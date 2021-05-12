<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

class LoraClient {
    private $cntx;
    private $ip;
    private $port;
    private $appeui;
    private $seq = 1;
    private $hbTid = 0;

    function __construct($cntx, $server, $appeui) {
        list($ip, $port) = explode(',', $server);
        if (!filter_var($ip, FILTER_VALIDATE_IP)) {
            $cntx->getLogger()->critical("lora server ip({$ip}) is invalid.");
            return;
        }

        $this->cntx = $cntx;
        $this->ip = $ip;
        $this->port = $port;
        $this->appeui = $appeui;
    }

    function nextSeq($reset = false) {
        if ($reset || $this->seq === PHP_INT_MAX) {
            $this->seq = 1;
        }
        return $this->seq++;
    }

    function connect() {
        if ($this->ip) {
            $this->client = new \swoole_client(SWOOLE_SOCK_TCP, SWOOLE_SOCK_ASYNC);
            $this->client->set([
                'open_length_check' => true,
                'package_length_offset' => 3,
                'package_length_type' => 'n',
                'package_body_offset' => 5
            ]);
            $this->client->on('Connect', [$this, 'onConnect']);
            $this->client->on('Receive', [$this, 'onReceive']);
            $this->client->on('Close', [$this, 'onClose']);
            $this->client->on('Error', [$this, 'onError']);
            $this->client->connect($this->ip, $this->port);
        }
    }

    function reconnect() {
        $serv = $this->cntx->getServ();
        if ($this->hbTid) {
            $serv->clearTimer($this->hbTid);
            $this->hbTid = 0;
        }
        $this->client = null;
        $serv->after(Config::LORA_RECONNECT_INTERVAL, [$this, 'connect']);
    }

    function send($data) {
        $logger = $this->cntx->getLogger();

        $data = json_encode($data);
        $logger->debug('LoraClient send json: ' . $data);
        $data = LoraUtils::pack($data);
        $logger->debug('LoraClient send bin: ' . bin2hex($data));

        $client = $this->client;
        if ($client) {
            $client->send($data);
        } else {
            $logger->debug('LoraClient is null.');
        }
    }

    function join() {
        $data = [
            'cmd' =>  'join',
            'cmdseq' => $this->nextSeq(true),
            'appeui' => $this->appeui,
        ];
        $this->send($data);
    }

    function heartbeat() {
        $data = [
            'cmd' => 'heartbeat',
            'appeui' => $this->appeui,
        ];
        $this->send($data);
    }

    public function sendto($address, $data) {
        $data = [
            'cmd' => 'sendto',
            'cmdseq' => $this->nextSeq(),
            'devaddr' => $address['devaddr'],
            'confirm' => false,
            'payload' => base64_encode($data),
        ];

        $this->send($data);
    }

    public function multicast($addresses, $data) {
        $data = [
            'cmd' => 'mcast',
            'cmdseq' => $this->nextSeq(),
            'devs' => $addresses,
            'confirm' => false,
            'payload' => base64_encode($data),
        ];

        $this->send($data);
    }

    function onConnect($client) {
        $logger = $this->cntx->getLogger();
        $logger->debug('LoraClient onConnect');

        $this->join();
    }

    function onReceive($client, $data) {
        $cntx = $this->cntx;
        $logger = $cntx->getLogger();
        $serv = $cntx->getServ();

        $logger->debug('LoraClient onReceive bin: ' . bin2hex($data));
        $data = LoraUtils::unpack($data);
        $logger->debug('LoraClient onReceive json: ' . $data);

        $data = json_decode($data);
        switch ($data->cmd) {
            case 'join_ack':
                if ($data->msg === 'JOIN ACCEPT') {
                    $this->hbTid = $serv->tick(Config::LORA_HEARTBEAT_INTERVAL, [$this, 'heartbeat']);
                }
                break;
            case 'updata':
            case 'dev_join':
                $payload = '';
                if (isset($data->payload)) {
                    $payload = $data->payload;
                }

                if (isset($data->crc) && !$data->crc) {
                    $logger->debug('crc: wrong.');
                    $payload = base64_decode($payload);

                    if (strlen($payload) < 2 + Utils::LEN_SIZE + Utils::SN_SIZE) {
                        $logger->debug('crc: payload too short.');
                        break;
                    }

                    $sn = Utils::unpackSn(substr($payload, 2 + Utils::LEN_SIZE));
                    $devMgr = $cntx->getDeviceMgr();
                    $device = $devMgr->getDevice($sn);
                    if ($device) {
                        $device->setPcdt(time());
                    } else {
                        $logger->debug("crc: device $sn does not exist.");
                    }

                    break;
                }

                $cntx->onPacket(
                    $serv,
                    base64_decode($payload),
                    ['devaddr' => $data->devaddr],
                    $data->gwinfo
                );
                break;
        }
    }

    function onClose($client) {
        $logger = $this->cntx->getLogger();
        $logger->debug(sprintf(
            'LoraClient onClose: %d(%s)', $client->errCode, socket_strerror($client->errCode)
        ));

        $this->reconnect();
    }

    function onError($client) {
        $logger = $this->cntx->getLogger();
        $logger->debug(sprintf(
            'LoraClient onError: %d(%s)', $client->errCode, socket_strerror($client->errCode)
        ));

        $this->reconnect();
    }
}
