<?php
class HttpServer {
    function __construct($ip, $port) {
        $serv = new swoole_http_server($ip, $port);
        $serv->on('WorkerStart', [$this, 'onWorkerStart']);
        $serv->on('Request', [$this, 'onRequest']);
        $this->serv = $serv;
    }

    function start() {
        $this->serv->start();
    }

    function onWorkerStart() {
        $client = new swoole_client(SWOOLE_SOCK_TCP, SWOOLE_SOCK_ASYNC);
        /*
        $client->set([
            'open_eof_check' => true,
            'package_eof' => 'hzw'
        ]);
        */
        $client->on('Connect', [$this, 'onEchoClientConnect']);
        $client->on('Receive', [$this, 'onEchoClientReceive']);
        $client->on('Close', [$this, 'onEchoClientClose']);
        $client->on('Error', [$this, 'onEchoClientError']);
        $client->connect('127.0.0.1', 9505);
        $this->client = $client;
    }

    function onRequest($req, $rep) {
        $reqUri = $req->server['request_uri'];
        
        echo "onRequest: $reqUri\n";

        if ($reqUri === '/favicon.ico') {
            $rep->status(404);
            $rep->end();
            return;
        }

        $this->client->send('0123hzw4567hzw');
        $this->rep = $rep;
    }

    function onEchoClientConnect($client) {
        echo "onEchoClientConnect\n";
    }

    function onEchoClientReceive($client, $data) {
        echo "onEchoClientReceive: $data\n";
        $this->rep->end("<h1>$data</h1>");
    }

    function onEchoClientClose($client) {
        echo "onEchoClientClose\n";
    }

    function onEchoClientError($client) {
        echo "onEchoClientError\n";
    }
}

$serv = new HttpServer('0.0.0.0', 9998);
$serv->start();
