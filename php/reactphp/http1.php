<?php
require __DIR__ . '/vendor/autoload.php';

use React\EventLoop\Factory;
use React\Http\Response;
use React\Http\Server;
use Psr\Http\Message\ServerRequestInterface;

$loop = Factory::create();

$server = new Server(function(ServerRequestInterface $request) {
    return new Response(200, ['Content-Type' => 'text/plain'], "Hello Zhiwen He\n");
});
$socket = new React\Socket\Server('0.0.0.0:8000', $loop);
$server->listen($socket);

$loop->run();
