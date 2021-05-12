<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

use Hzw\Cmd\Status;

class LoraServer {
    private $serv;
    private $logger;
    private $device;
    private $devaddr;

    function __construct($ip, $port, $logger) {
        $this->serv = new \swoole_server($ip, $port, SWOOLE_PROCESS, SWOOLE_SOCK_TCP);
        $this->serv->set([
            'open_length_check' => true,
            'package_length_offset' => 3,
            'package_length_type' => 'n',
            'package_body_offset' => 5
        ]);
        $this->serv->on('WorkerStart', [$this, 'onWorkerStart']);
        $this->serv->on('Connect', [$this, 'onConnect']);
        $this->serv->on('Receive', [$this, 'onReceive']);
        $this->serv->on('Close', [$this, 'onClose']);

        $this->logger = $logger;
    }

    function setDevice($device) {
        $this->device = $device;
        $device->setSendCb([$this, 'sendUpdata']);
    }

    function setDevaddr($devaddr) {
        $this->devaddr = $devaddr;
    }

    function start() {
        $this->serv->start();
    }

    function sendTo($fd, $cmd, $data) {
        $data = [
            'cmd' => $cmd,
            'cmdseq' => $this->device->nextSeq(),
            'devaddr' => sprintf('%04d', $this->devaddr),
            'payload' => base64_encode($data),
            'gwinfo' => ['rssi' => -50],
            //'crc' => 0
        ];
        $data = json_encode($data);
        $this->logger->debug("LoraServer send json: $data");
        $data = LoraUtils::pack($data);
        $this->logger->debug("LoraServer send bin: " . bin2hex($data));

        $this->serv->send($fd, $data);
    }

    function sendToAll($cmd, $data) {
        foreach ($this->serv->connections as $fd) {
            $this->sendTo($fd, $cmd, $data);
        }
    }

    function sendUpdata($data) {
        $this->sendToAll('updata', $data);
    }

    function onWorkerStart($serv, $workerId) {
        $this->device->start();
    }

    function onConnect($serv, $fd, $reactorId) {
        $this->sendTo($fd, 'dev_join', $this->device->devJoin());
    }

    function onReceive($serv, $fd, $reactorId, $data) {
        $logger = $this->logger;

        $logger->debug("LoraServer onReceive bin: " . bin2hex($data));
        $data = LoraUtils::unpack($data);
        $logger->debug("LoraServer onReceive json: $data");

        $data = json_decode($data);
        switch ($data->cmd) {
            case 'join':
                break;
            case 'sendto':
            case 'mcast':
                $this->device->onReceive([], base64_decode($data->payload));
                break;
        }
    }

    function onClose($serv, $fd, $reactorId) {
        echo 'onClose' . PHP_EOL;
    }
}
