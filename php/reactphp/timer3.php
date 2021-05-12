<?php
require __DIR__ . '/vendor/autoload.php';

$loop = React\EventLoop\Factory::create();
$loop->addPeriodicTimer(1, function() {
    echo "3\n";
});
$loop->addPeriodicTimer(1, function() {
    echo "4\n";
});
$loop->addTimer(2, function() {
    echo "1\n";
});
$loop->addTimer(2, function() {
    echo "2\n";
});
$loop->addTimer(4, function() {
    echo "1\n";
});
$loop->addTimer(4, function() {
    echo "2\n";
});
$loop->run();
