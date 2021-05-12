<?php
use Swoole\Coroutine as co;

$serv = new Swoole\Http\Server('127.0.0.1', 9501);

$serv->on('Request', function($req, $res) {
    static $count = 0;
    $uri = $req->server['request_uri'];
    echo "$uri\n";
    if ($uri === '/favicon.ico') {
        $res->status(404);
        $res->end();
        return;
    }
    //if ($count % 2 !== 0) {
    if ($count++ % 2 !== 0) {
        echo "before co::sleep\n";
        co::sleep(10);
        echo "after co::sleep\n";
    }
    $res->end('hello zhiwenhe.');
    //$count++;
});

$serv->start();
