<?php
require __DIR__ . '/vendor/autoload.php';

$loop = React\EventLoop\Factory::create();
$stream = new React\Stream\ReadableResourceStream(fopen('./timer1.php', 'r'), $loop, 1);
$stream->on('data', function($data) use ($stream, $loop) {
    echo "$data\n";
    $stream->pause();
    $loop->addTimer(1, function() use ($stream) {
        $stream->resume();
    });
});
$stream->on('end', function() {
    echo "end\n";
});
$stream->on('close', function() {
    echo "close\n";
});
$loop->run();
echo "finish\n";
