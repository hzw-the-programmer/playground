<?php
require __DIR__ . '/vendor/autoload.php';

use React\EventLoop\Factory;
use React\Http\Server;

$loop = Factory::create();
$videoStreaming = new VideoStreaming($loop);
$server = new Server($videoStreaming);
$socket = new React\Socket\Server('0.0.0.0:8000', $loop);
$server->listen($socket);
$loop->run();
