<?php
require __DIR__ . '/vendor/autoload.php';

use React\EventLoop\Factory;
use React\Socket\ConnectionInterface;
use React\Socket\Server;

$loop = Factory::create();

$server = new Server('0.0.0.0:8000', $loop);
$server->on('connection', function(ConnectionInterface $connection) {
    echo date('Y-m-d H:i:s') . " {$connection->getRemoteAddress()} ***connection***\n";
    $connection->write("Hi\n");
    $connection->on('data', function($data) use ($connection) {
        echo date('Y-m-d H:i:s') . " {$connection->getRemoteAddress()} $data";
        $connection->write(strtoupper($data));
    });
    $connection->on('end', function() use ($connection) {
        echo date('Y-m-d H:i:s') . " {$connection->getRemoteAddress()} ***end***\n";
    });
    $connection->on('error', function(Exception $e) use ($connection) {
        echo date('Y-m-d H:i:s') . " {$connection->getRemoteAddress()} ***error***\n";
    });
    $connection->on('close', function() use ($connection) {
        echo date('Y-m-d H:i:s') . " {$connection->getRemoteAddress()} ***close***\n";
    });
});
echo "Listening on {$server->getAddress()}\n";

$loop->run();
