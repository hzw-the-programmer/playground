<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

use Swoole\Client;
use Hzw\Cmd\HeartBeat;
use Hzw\Cmd\Data;
use Hzw\Cmd\Status;
use Hzw\Cmd\StatusData;

class VirtualDevices {
    const HEART_BEAT_INTERVAL = 60 * 1000;
    const DATA_INTERVAL = 600 * 1000;
    const STATUS_INTERVAL = 60 * 1000;

    public $client;
    public $devices;

    function __construct($port) {
        $this->devices = static::getDevicesFromDb();

        $this->client = new Client(SWOOLE_SOCK_UDP, SWOOLE_SOCK_ASYNC);
        $this->client->set([
            'bind_address' => '',
            'bind_port' => $port
        ]);
        $this->client->on('Connect', [$this, 'onConnect']);
        $this->client->on('Receive', [$this, 'onReceive']);
    }

    function onConnect($client) {
        swoole_timer_after((60 - time() % 60) * 1000, function() {
            swoole_timer_tick(static::HEART_BEAT_INTERVAL, [$this, 'sendHeartBeat']);
            swoole_timer_tick(static::DATA_INTERVAL, [$this, 'sendData']);
            swoole_timer_tick(static::STATUS_INTERVAL, [$this, 'sendStatus']);
        });
    }

    function onReceive($client, $data) {

    }

    function connect($ip, $port) {
        $this->client->connect($ip, $port);
    }

    function sendHeartBeat() {
        $dt = time();
        foreach ($this->devices as $device) {
            $sn = $device->getSn();
            $seq = $device->nextSeq();
            $header = new Header([$sn], $seq, $dt, 'T');

            $body = new HeartBeat(0x96);

            $packet = new Packet($header, $body);
            $this->client->send($packet->pack());
        }
    }

    function sendData() {
        static $data = 0;
        $dt = time();
        foreach ($this->devices as $device) {
            $sn = $device->getSn();
            $seq = $device->nextSeq();
            $header = new Header([$sn], $seq, $dt, 'T');

            $channels = $device->getChannels();
            foreach ($channels as $channel) {
                $channel->setData($data);
            }
            $body = new Data($channels);

            $packet = new Packet($header, $body);
            $this->client->send($packet->pack());
        }
        $data++;
    }

    function sendStatus() {
        static $index = 0;
        $dt = time();
        foreach ($this->devices as $device) {
            $sn = $device->getSn();
            $seq = $device->nextSeq();
            $header = new Header([$sn], $seq, $dt, 'T');

            $channels = $device->getChannels();
            foreach ($channels as $channel) {
                switch ($channel->getType()) {
                    case Channel::WS:
                        $statuses = [0x80, 0x50, 0x40, 0x20, 0x00];
                        break;
                    default:
                        $statuses = [0x20, 0x00];
                        break;
                }
                $channel->setStatus($statuses[$index % count($statuses)]);
                $channel->setData(0.8);
            }
            //$body = new Status($channels);
            $body = new StatusData($channels);

            $packet = new Packet($header, $body);
            $this->client->send($packet->pack());
        }
        $index++;
    }

    static function getDevicesFromDb() {
        $devices = [];

        try {
            $dbh = new \PDO(Config::DSN, Config::DB_USER, Config::DB_PW);
            $stmt = $dbh->query('
                SELECT
                di.id AS deviceId, di.sn,
                ci.id AS channelId, ci.slot, ci.port, ci.type
                FROM mpoint AS mp
                LEFT JOIN channels_info AS ci
                ON mp.ciid = ci.id
                LEFT JOIn devices_info AS di
                ON ci.device_id = di.id
                WHERE mp.endtime = 0
                ORDER BY di.sn, ci.slot, ci.port, ci.type
            ');
            while ($row = $stmt->fetch(\PDO::FETCH_ASSOC)) {
                $sn = $row['sn'];
                if (!isset($devices[$sn])) {
                    $devices[$sn] = new Device($sn);
                }
                $device = $devices[$sn];

                $slot = $row['slot'];
                $port = $row['port'];
                $type = $row['type'];
                $channel = new Channel($slot, $port, $type);

                $device->addChannel($channel);
            }
        } catch (\PDOException $e) {
            print_r($e->errorInfo);
        }

        return $devices;
    }
}
