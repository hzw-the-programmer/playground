<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
require __DIR__ . '/vendor/autoload.php';

use Monolog\Logger;
use Monolog\Handler\StreamHandler;
use Hzw\Config;
use Hzw\LoraServer;
use Hzw\VirtualDevice;

if ($argc < 5) {
    die("{$argv[0]} ip port devaddr sn...\n");
}

$ip = $argv[1];
$port = $argv[2];
$devaddr = $argv[3];

$sns = [];
for ($i = 4; $i < $argc; $i++) {
    $sns[] = $argv[$i];
}

$logfile = '/tmp/iot/vd' . getmypid();
$handler = new StreamHandler($logfile, Config::LOGGER_LEVEL);
$logger = new Logger('main');
$logger->pushHandler($handler);

$device = new VirtualDevice($sns, $logger);

$serv = new LoraServer($ip, $port, $logger);
$serv->setDevice($device);
$serv->setDevaddr($devaddr);

$serv->start();
