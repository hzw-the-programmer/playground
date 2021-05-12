<?php
require __DIR__ . '/vendor/autoload.php';

use React\Datagram\Socket;
use React\EventLoop\Factory;
use React\Stream\WritableResourceStream;

$address = '0.0.0.0:8000';

$loop = Factory::create();

$stdout = new WritableResourceStream(STDOUT, $loop);

$factory = new React\Datagram\Factory($loop);
$factory
    ->createServer($address)
    ->then(
        function(Socket $server) use ($stdout) {
            $server->on('message', function($message, $address, $server) use ($stdout) {
                $stdout->write("client $address: $message");
                $server->send("$address echo: $message", $address);
            });
        },
        function(Exception $e) use ($stdout) {
            $stdout->write($e->getMessage());
        }
    );

$stdout->write("Listening on $address\n");

$loop->run();
