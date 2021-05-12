<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

use Hzw\Cmd\SetSn;
use Hzw\Cmd\GetSn;
use Hzw\Cmd\SetWorkTime;
use Hzw\Cmd\GetWorkTime;
use Hzw\Cmd\Calibration;
use Hzw\Cmd\DeviceStandbyHB;
use Hzw\Cmd\Ack;
use Hzw\Cmd\StatusData;
use Hzw\Cmd\Data;
use Hzw\Cmd\PowerOff;
use Hzw\Cmd\PowerOn;
use Hzw\Cmd\DeviceInfo;
use Hzw\Cmd\HeartBeatWithData;
use Hzw\Cmd\Open;
use Hzw\Cmd\Close;
use Hzw\Cmd\Status;
use Hzw\Cmd\GetParam;
use Hzw\Cmd\SetParam;
use Hzw\Cmd\HeartBeat;

class VirtualDevice {
    const HEART_BEAT_INTERVAL = 5 * 1000;
    const DATA_INTERVAL = 5 * 1000;
    const STATUS_INTERVAL = 5 * 1000;

    private $devices;
    private $logger;
    private $sendCb;

    function __construct($sns, $logger) {
        $this->devices = [];

        $sql = '
            SELECT
                di.id AS did, di.sn, dip.sn AS psn, di.ip, di.port, di.devaddr, di.type,
                ci.id AS cid, ci.slot, ci.port AS cport, ci.type AS ctype,
                mp.id AS mid, mp.pid, mp.name,
                mprs.raw_status AS status, mprs.time AS dt, mprs.swiftnum AS seq,
                mpps.raw_status AS preStatus, mpps.time AS preDt, mpps.swiftnum AS preSeq
            FROM devices_info AS di
            LEFT JOIN channels_info AS ci
            ON di.id = ci.device_id
            LEFT JOIN mpoint AS mp
            ON ci.id = mp.ciid AND mp.endtime = 0
            LEFT JOIN mpoint_realtime_status AS mprs
            ON mp.id = mprs.mpoint_id
            LEFT JOIN mpoint_pre_status AS mpps
            ON mp.id = mpps.mpoint_id
            LEFT JOIN devices_info AS dip
            ON di.parent_id = dip.id
            WHERE di.sn = :sn
        ';
        try {
            $dbh = new \PDO(Config::DSN, Config::DB_USER, Config::DB_PW, [
                \PDO::ATTR_ERRMODE => \PDO::ERRMODE_EXCEPTION,
            ]);
            $stmt = $dbh->prepare($sql);
            for ($i = 0; $i < count($sns); $i++) {
                $sn = $sns[$i];
                $stmt->bindParam(':sn', $sn);
                $stmt->execute();
                $device = null;
                while ($row = $stmt->fetch(\PDO::FETCH_ASSOC)) {
                    if (!$device) {
                        $device = Device::createFromDb($row);
                    }
                    if ($row['mid'] && !$device->getChannel($row['slot'], $row['cport'])) {
                        $device->addChannel(Channel::createFromDb($row));
                    }
                }
                if (!$device) {
                    $device = new Device($sn);
                }
                if ($i !== 0) {
                    $device->setPsn($sns[0]);
                }
                $this->devices[$sn] = $device;
            }
        } catch (\PDOException $e) {
            $logger->debug('VirtualDevice init db error. ' . $e->getMessage());
        }

        $this->logger = $logger;
        Packet::init();
    }

    function setSendCb($sendCb) {
        $this->sendCb = $sendCb;
    }

    function getSns() {
        $sns = [];
        foreach ($this->devices as $device) {
            $sn = $device->getSn();
            $psn = $device->getPsn();
            if ($psn) {
                $sns[] = $sn;
            } else {
                array_unshift($sns, $sn);
            }
        }
        return $sns;
    }

    function nextSeq() {
        return $this->devices[$this->getSns()[0]]->nextSeq();
    }

    function start() {
        //swoole_timer_after((60 - time() % 60) * 1000, function() {
            //$this->sendHeartBeat();
            //swoole_timer_tick(self::HEART_BEAT_INTERVAL, [$this, 'sendHeartBeat']);

            //$this->sendHeartBeatWithData();
            //swoole_timer_tick(self::HEART_BEAT_INTERVAL, [$this, 'sendHeartBeatWithData']);

            //$this->sendData();
            //swoole_timer_tick(self::DATA_INTERVAL, [$this, 'sendData']);

            //$this->sendStatus();
            //swoole_timer_tick(self::STATUS_INTERVAL, [$this, 'sendStatus']);

            //$this->sendCalibration();
            //swoole_timer_tick(self::HEART_BEAT_INTERVAL, [$this, 'sendCalibration']);

            $this->sendDeviceStandbyHB();
            swoole_timer_tick(self::HEART_BEAT_INTERVAL, [$this, 'sendDeviceStandbyHB']);
        //});
    }

    function onReceive($source, $data) {
        $logger = $this->logger;

        $pcdt = time();
        $packet = Packet::unpack($data);
        $packet->setSource($source)
               ->setPcdt($pcdt);

        $logger->debug('onReceive: ' . $packet . ' ' . bin2hex($data));

        if (!$packet->isValid()) return;

        $header = $packet->getHeader();
        $body = $packet->getBody();

        $sns = $header->getSns();
        $psn = $this->getSns()[0];
        if ($sns[0] === '00000000000000') {
            $sns[0] = $psn;
        }
        $sn = $sns[count($sns) - 1];

        if ($sns[0] !== $psn || !isset($this->devices[$sn])) {
            return;
        }

        $device = $this->devices[$sn];

        $b = null;
        switch ($body::CMD) {
            case GetSn::CMD:
                $b = new Ack($header->getSeq(), 0x00, $body::CMD, $this->getSns());
                break;
            case SetSn::CMD:
                $newSn = $body->getSns()[0];
                $OldDevice = null;
                foreach ($this->devices as $device) {
                    $sn = $device->getSn();
                    $psn = $device->getPsn();
                    if ($psn) {
                        $device->setPsn($newSn);
                    } else {
                        $oldDevice = $device;
                    }
                }
                $this->devices[$newSn] = $oldDevice->clone($newSn);
                unset($this->devices[$oldDevice->getSn()]);
                $b = new Ack($header->getSeq(), 0x00, $body::CMD, null);
                break;
            case Open::CMD:
                foreach ($body->getChannels() as $channel) {
                    switch ($channel->getSlot()) {
                        case 3:
                            $channel->setType(Channel::GND_L);
                            break;
                        case 4:
                            $channel->setType(Channel::WS);
                            break;
                        default:
                            $channel->setType(Channel::GND_H);
                            break;
                    }
                    if (!$device->getChannel($channel->getSlot(), $channel->getPort())) {
                        $device->addChannel($channel);
                    }
                }
                $b = new Ack($header->getSeq(), 0x00, $body::CMD, null);
                break;
            case Close::CMD:
                foreach ($body->getChannels() as $channel) {
                    $device->removeChannel($channel);
                }
                $b = new Ack($header->getSeq(), 0x00, $body::CMD, null);
                break;
            case GetParam::CMD:
                $type = $body->getType();
                $param = $device->getParam($type);
                if (!$param) {
                    switch ($type) {
                        case Channel::GND_H:
                            $param = new Param(Channel::GND_H, 600, 0, 10, 5, 600, 1200, 1800);
                            break;
                        case Channel::GND_L:
                            $param = new Param(Channel::GND_L, 600, 0, 4, 5, 600, 1200, 1800);
                            break;
                        default:
                            $param = new Param(Channel::WS, 600, 0.75, 35, 5, 600, 1200, 1800);
                            break;
                    }
                }
                $b = new Ack($header->getSeq(), 0x00, $body::CMD, $param);
                break;
            case SetParam::CMD:
                $param = $body->getParam();
                $device->setParam($param->getType(), $param);
                $b = new Ack($header->getSeq(), 0x00, $body::CMD, null);
                break;
            case GetWorkTime::CMD:
                $b = new Ack($header->getSeq(), 0x00, $body::CMD, $device->getPeriods());
                break;
            case SetWorkTime::CMD:
                $device->setPeriods($body->getPeriods());
                $b = new Ack($header->getSeq(), 0x00, $body::CMD, null);
                break;
        }

        if ($b) {
            $h = new Header($sns, 0x00, time(), 'T');
            $this->send((new Packet($h, $b))->pack());
        }
    }

    function send($data) {
        $this->logger->debug('send: ' . Packet::unpack($data) . ' ' . bin2hex($data));
        call_user_func($this->sendCb, $data);
    }

    function sendHeartBeat() {
        $dt = time();

        foreach ($this->devices as $device) {
            $sn = $device->getSn();
            $psn = $device->getPsn();
            $sns = [$sn];
            if ($psn) {
                array_unshift($sns, $psn);
            }

            $header = new Header($sns, $device->nextSeq(), $dt, 'T');
            $body = new HeartBeat(0x96);
            $packet = new Packet($header, $body);

            $this->send($packet->pack());
        }
    }

    function sendHeartBeatWithData() {
        static $index = 0;
        $dt = time();

        foreach ($this->devices as $device) {
            if (!($channels = $device->getChannels())) continue;
            foreach ($channels as $channel) {
                $statuses = [0x00, 0x20];
                if ($channel->getType() === Channel::WS) {
                    array_push($statuses, 0x40, 0x50, 0x80);
                }
                //$channel->setStatus($statuses[$index % count($statuses)]);
                $channel->setStatus(0x20);
                $channel->setData(0.8);
                $channel->setAvgData($index);
            }
            $sn = $device->getSn();
            $psn = $device->getPsn();
            $sns = [$sn];
            if ($psn) {
                array_unshift($sns, $psn);
            }

            $header = new Header($sns, $device->nextSeq(), $dt, 'T');
            $body = new HeartBeatWithData($index, $channels);
            $packet = new Packet($header, $body);

            $round = intval($index / 10);
            $count = $index % 10;
            $send = true;

            $rounds = [1, 3, 5, 7, 9];
            if (array_search($round, $rounds) !== false && $count > 3) {
                $send = false;
            } 

            if ($send) {
                $this->send($packet->pack());
            }
        }
        $index++;
    }

    function sendData() {
        $dt = time();

        foreach ($this->devices as $device) {
            if (!($channels = $device->getChannels())) continue;
            $sn = $device->getSn();
            $psn = $device->getPsn();
            $sns = [$sn];
            if ($psn) {
                array_unshift($sns, $psn);
            }

            $header = new Header($sns, $device->nextSeq(), $dt, 'T');
            $body = new Data($channels);
            $packet = new Packet($header, $body);

            $this->send($packet->pack());
        }
    }

    function sendStatus() {
        static $index = 0;
        $dt = time();

        foreach ($this->devices as $device) {
            if (!($channels = $device->getChannels())) continue;
            foreach ($channels as $channel) {
                $statuses = [0x00, 0x20];
                if ($channel->getType() === Channel::WS) {
                    array_push($statuses, 0x40, 0x50, 0x80);
                }
                $channel->setStatus($statuses[$index % count($statuses)]);
                $channel->setData(0.8);
            }
            $sn = $device->getSn();
            $psn = $device->getPsn();
            $sns = [$sn];
            if ($psn) {
                array_unshift($sns, $psn);
            }

            $header = new Header($sns, $device->nextSeq(), $dt, 'T');
            //$body = new Status($channels);
            $body = new StatusData($channels);
            $packet = new Packet($header, $body);

            $this->send($packet->pack());
        }
        $index++;
    }

    function sendCalibration() {
        $dt = time();

        foreach ($this->devices as $device) {
            if (!($channels = $device->getChannels())) continue;
            $sn = $device->getSn();
            $psn = $device->getPsn();
            $sns = [$sn];
            if ($psn) {
                array_unshift($sns, $psn);
            }

            $header = new Header($sns, $device->nextSeq(), $dt, 'T');
            $body = new Calibration($channels);
            $packet = new Packet($header, $body);

            $this->send($packet->pack());
        }
    }

    function sendDeviceStandbyHB() {
        $dt = time();

        foreach ($this->devices as $device) {
            $sn = $device->getSn();
            $psn = $device->getPsn();
            $sns = [$sn];
            if ($psn) {
                array_unshift($sns, $psn);
            }

            $header = new Header($sns, $device->nextSeq(), $dt, 'T');
            $body = new DeviceStandbyHB();
            $packet = new Packet($header, $body);

            $this->send($packet->pack());
        }
    }

    function devJoin() {
        $sn = $this->getSns()[0];
        $device = $this->devices[$sn];
        $dt = time();
        $header = new Header([$sn], $device->nextSeq(), $dt, 'T');
        $body = new DeviceInfo('3.4.5');
        $packet = new Packet($header, $body);

        return $packet->pack();
    }
}
