<?php
require __DIR__ . '/vendor/autoload.php';

$loop = React\EventLoop\Factory::create();
$count = 0;

$loop->addPeriodicTimer(1, function($timer) use (&$count, $loop) {
    echo $count++ . "\n";
    if ($count === 5) {
        $loop->cancelTimer($timer);
    }
});

$loop->run();
echo "finish\n";
