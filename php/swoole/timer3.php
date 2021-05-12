<?php
require __DIR__ . '/vendor/autoload.php';

use Monolog\Logger;
use Monolog\Handler\StreamHandler;

$logger = new Logger('main');
$logger->pushHandler(new StreamHandler('/tmp/timer3', Logger::DEBUG));

swoole_timer_tick(1000, function($timerId) use ($logger) {
    $logger->debug($timerId);
});
