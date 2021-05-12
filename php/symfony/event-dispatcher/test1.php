<?php
require_once './vendor/autoload.php';

use Symfony\Component\EventDispatcher\EventDispatcher;

$dispatcher = new EventDispatcher();
$dispatcher->addListener('hzw.event1', function ($event, $eventName) {
    echo "hzw.event1 handler 0 0\n";
});
$dispatcher->addListener('hzw.event1', function ($event, $eventName) {
    echo "hzw.event1 handler 0 1\n";
});
$dispatcher->addListener('hzw.event1', function ($event, $eventName) {
    echo "hzw.event1 handler 0 2\n";
});
$dispatcher->addListener('hzw.event1', function ($event, $eventName) {
    echo "hzw.event1 handler 1 0\n";
}, 1);
$dispatcher->addListener('hzw.event1', function ($event, $eventName) {
    echo "hzw.event1 handler 1 1\n";
}, 1);

$dispatcher->addListener('hzw.event2', function ($event, $eventName) {
    echo "hzw.event2 handler 0 0\n";
});
$dispatcher->addListener('hzw.event2', function ($event, $eventName) {
    echo "hzw.event2 handler 0 1\n";
    $event->stopPropagation();
});
$dispatcher->addListener('hzw.event2', function ($event, $eventName) {
    echo "hzw.event2 handler 0 2\n";
});
$dispatcher->addListener('hzw.event2', function ($event, $eventName) {
    echo "hzw.event2 handler 1 0\n";
}, 1);
$dispatcher->addListener('hzw.event2', function ($event, $eventName) {
    echo "hzw.event2 handler 1 1\n";
}, 1);

$dispatcher->dispatch('hzw.event1');
echo "***********************\n";
$dispatcher->dispatch('hzw.event2');