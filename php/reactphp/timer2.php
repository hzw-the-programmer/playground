<?php
require __DIR__ . '/vendor/autoload.php';

$loop = React\EventLoop\Factory::create();
$count = 0;

$timer = $loop->addPeriodicTimer(1, function() use (&$count) {
    echo $count++ . "\n";
});

$loop->addTimer(5, function() use ($loop, $timer) {
    $loop->cancelTimer($timer);
});

$loop->run();
echo "done\n";
