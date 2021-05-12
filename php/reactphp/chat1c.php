<?php
require __DIR__ . '/vendor/autoload.php';

use React\EventLoop\Factory;
use React\Socket\ConnectionInterface;
use React\Socket\Connector;
use React\Stream\ReadableResourceStream;
use React\Stream\WritableResourceStream;

$loop = Factory::create();

$stdin = new ReadableResourceStream(STDIN, $loop);
$stdout = new WritableResourceStream(STDOUT, $loop);

$connector = new Connector($loop);
$connector
    ->connect('127.0.0.1:8000')
    ->then(
        function(ConnectionInterface $connection) use ($stdin, $stdout) {
            $stdin->pipe($connection)->pipe($stdout);
            /*
            $connection->on('data', function($data) use ($stdout) {
                $stdout->write($data);
            });
            $stdin->on('data', function($data) use ($connection) {
                $connection->write($data);
            });
            */
        },
        function(Exception $e) use ($stdout) {
            $stdout->write($e->getMessage());
        }
    );

$loop->run();
