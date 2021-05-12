<?php
require __DIR__ . '/vendor/autoload.php';

use React\EventLoop\Factory;

$loop = Factory::create();

$stream = new LineStream(fopen('./stream3.php', 'r'), $loop);
$stream->on('line', function($line) {
    echo "$line\n";
});

$loop->run();
