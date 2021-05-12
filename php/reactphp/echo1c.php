<?php
require __DIR__ . '/vendor/autoload.php';

use React\Datagram\Socket;
use React\EventLoop\Factory;
use React\Stream\ReadableResourceStream;
use React\Stream\WritableResourceStream;

$address = '127.0.0.1:8000';

$loop = Factory::create();

$stdin = new ReadableResourceStream(STDIN, $loop);
$stdout = new WritableResourceStream(STDOUT, $loop);

$factory = new React\Datagram\Factory($loop);
$factory
    ->createClient($address)
    ->then(
        function(Socket $client) use ($stdin, $stdout) {
            $stdin->on('data', function($data) use ($client) {
                $client->send($data);
            });
            $client->on('message', function($message) use ($stdout) {
                $stdout->write($message);
            });
        },
        function(Exception $e) use ($stdout) {
            $stdout->write($e->getMessage());
        }
    );

$loop->run();
