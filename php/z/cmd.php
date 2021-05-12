<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
require __DIR__ . '/vendor/autoload.php';

use Hzw\Protocal;

$pkt = hex2bin('503300230df661fe01480000000004bd12051c09202641020100085515c5430101085555cd439a');
list($valid, $ver, $len, $sn, $id, $dt, $cmd, $body) = Protocal::unpack($pkt);
$body = bin2hex($body);

echo "$valid\n";
echo "$ver\n";
echo "$len\n";
echo "$sn\n";
echo "$id\n";
echo "$dt\n";
echo "$cmd\n";
echo "$body\n";
