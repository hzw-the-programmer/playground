<?php
$serv = new Swoole\Http\Server('0.0.0.0', 9501);
$serv->on('Request', function($req, $res) {
    $res->end('<h1>Hello Swoole. #' . rand(1000, 9999) . '</h1>');
});

$serv->start();

