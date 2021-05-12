<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
require __DIR__ . '/vendor/autoload.php';

use Monolog\Logger;
use Monolog\Handler\StreamHandler;
use Hzw\Config;
use Hzw\UdpClient;
use Hzw\VirtualDevice;

if ($argc < 5) {
    die("{$argv[0]} remote_host remote_port local_port sn...\n");
}

$remoteHost = $argv[1];
$remotePort = $argv[2];
$localPort = $argv[3];

$sns = [];
for ($i = 4; $i < $argc; $i++) {
    $sns[] = $argv[$i];
}

$logfile = '/tmp/iot/vd' . getmypid();
$handler = new StreamHandler($logfile, Config::LOGGER_LEVEL);
$logger = new Logger('main');
$logger->pushHandler($handler);

$device = new VirtualDevice($sns, $logger);

$client = new UdpClient($localPort, $logger);
$client->setDevice($device);

$client->connect($remoteHost, $remotePort);
