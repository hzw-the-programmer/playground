<?php
require __DIR__ . '/vendor/autoload.php';

use Psr\Http\Message\ServerRequestInterface;
use React\EventLoop\Factory;
use React\Http\Response;
use React\Http\Server;

$loop = Factory::create();

$socket = new React\Socket\Server('0.0.0.0:8000', $loop);

$loggingMiddleware = function(ServerRequestInterface $request, callable $next) {
    echo date('Y-m-d H:i:s') . ' ' . $request->getMethod() . ' ' . $request->getUri() . PHP_EOL;
    return $next($request);
};

$server = new Server([
    $loggingMiddleware,
    function(ServerRequestInterface $request) {
        return new Response(200, ['Content-Type' => 'text/plain'], 'Hello New World!');
    }
]);

$server->listen($socket);
echo 'Listening on ' . str_replace('tcp', 'http', $socket->getAddress()) . PHP_EOL;

$loop->run();
