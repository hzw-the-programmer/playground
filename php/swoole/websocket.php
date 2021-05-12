<?php
$serv = new Swoole\WebSocket\Server("0.0.0.0", 9501, SWOOLE_PROCESS, SWOOLE_TCP | SWOOLE_SSL);

$serv->addListener("0.0.0.0", 9502, SWOOLE_TCP | SWOOLE_SSL);

$serv->set([
    "ssl_cert_file" => "./localhost.crt",
    "ssl_key_file" => "./localhost.key"
]);

$serv->on("Open", function($serv, $req) {
    echo "handshake success with client {$req->fd}\n";
});

$serv->on("Message", function($serv, $frame) {
    echo "receive from client {$frame->fd}: {$frame->data}\n";
    $serv->push($frame->fd, "{$frame->data} from server");
});

$serv->on("Close", function($serv, $fd) {
    echo "client {$fd} closed\n";
});

$serv->on("Request", function($req, $res) {
    $res->sendfile("./websocket.html");
});

$serv->start();
