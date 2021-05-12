<?php
require 'vendor/autoload.php';

$sum = 0;
for ($count = 0; $count < 100; $count++) {
    $st = microtime(true);
    //$predis = new Predis\Client();
    /*
    $predis = new Predis\Client([
        'scheme' => 'tcp',
        'host' => '127.0.0.1',
        'port' => 6379
    ]);
    */
    $predis = new Predis\Client('tcp:\\127.0.0.1:6397');
    $predis->set('20180123000001', -88);
    $sum += microtime(true) - $st;
}
$avg = $sum / $count;
$w = max(strlen('sum'), strlen('count'), strlen('avg'));
echo sprintf("%{$w}s: %s\n", 'sum', $sum);
echo sprintf("%{$w}s: %s\n", 'count', $count);
echo sprintf("%{$w}s: %s\n", 'avg', $avg);
