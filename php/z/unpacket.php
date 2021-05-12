<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
require __DIR__ . '/vendor/autoload.php';

use Hzw\Packet;

if ($argc != 2) {
    exit("${argv[0]} packet\n");
}

echo Packet::unpack(hex2bin($argv[1]));