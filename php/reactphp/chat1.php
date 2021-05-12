<?php
require __DIR__ . '/vendor/autoload.php';

use React\EventLoop\Factory;
use React\Socket\ConnectionInterface;
use React\Socket\Server;

$loop = Factory::create();

$pool = new ConnectionPool();

$socket = new Server('0.0.0.0:8000', $loop);
$socket->on('connection', function(ConnectionInterface $connection) use ($pool) {
    $pool->add($connection);
});
echo "Listening on {$socket->getAddress()}\n";

$loop->run();
