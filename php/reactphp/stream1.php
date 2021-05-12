<?php
require __DIR__ . '/vendor/autoload.php';

$loop = React\EventLoop\Factory::create();
$stream = new React\Stream\ReadableResourceStream(fopen('./timer1.php', 'r'), $loop);
$stream->on('data', function($data) {
    echo "******\n";
    echo "$data\n";
    echo "******\n";
});
$stream->on('end', function() use ($stream) {
    echo "end\n";
    var_dump($stream->isReadable());
});
$stream->on('close', function() use ($stream) {
    echo "close\n";
    var_dump($stream->isReadable());
});

$loop->run();
