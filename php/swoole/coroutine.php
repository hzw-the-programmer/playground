<?php
echo getmypid() . "\n";

$serv = new Swoole\Http\Server("0.0.0.0", 8888, SWOOLE_BASE);

$serv->on("Request", function($request, $response) {
    echo getmypid() . "\n";
    sleep(10);
    //Swoole\Coroutine::sleep(10);
    $response->end("Hello Zhiwen He.");
});

$serv->start();
