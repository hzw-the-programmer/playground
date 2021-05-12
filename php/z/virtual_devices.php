<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
require __DIR__ . '/vendor/autoload.php';

use Hzw\VirtualDevices;

if($argc !== 4) {
    die("php virtual_devices.php remoteIp remotePort port\n");
}
$remoteIp = $argv[1];
$remotePort = $argv[2];
$port = $argv[3];

$vds = new VirtualDevices($port);
$vds->connect($remoteIp, $remotePort);
