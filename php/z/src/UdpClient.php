<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

use Swoole\Client;

class UdpClient {
    private $client;
    private $logger;
    private $device;

    function __construct($port, $logger) {
        $this->client = new Client(SWOOLE_SOCK_UDP, SWOOLE_SOCK_ASYNC);
        $this->client->set([
            'bind_address' => '',
            'bind_port' => $port
        ]);
        $this->client->on('Connect', [$this, 'onConnect']);
        $this->client->on('Receive', [$this, 'onReceive']);

        $this->logger = $logger;
    }

    function setDevice($device) {
        $this->device = $device;
        $device->setSendCb([$this, 'send']);
    }

    function send($data) {
        $this->client->send($data);
    }

    function connect($host, $port) {
        $this->client->connect($host, $port);
    }

    function onConnect($client) {
        $this->device->start();
    }

    function onReceive($client, $data) {
        $peer = $client->getPeerName();
        $source = ['ip' => $peer['host'], 'port' => $peer['port']];
        $this->device->onReceive($source, $data);
    }
}
