<?php
echo date("Y-m-d H:i:s") . "\n";

swoole_process::signal(SIGTERM, function($signo) {
    echo "signo: $signo\n";
});

//Swoole\Timer::tick(5000, function($timerId) {
swoole_timer_tick(5000, function($timerId) {
   echo date("Y-m-d H:i:s") . "\n";
});

Swoole\Event::cycle(function() {
    echo "cycle b\n";
}, true);

Swoole\Event::cycle(function() {
    echo "cycle a\n";
});

function defer_cb() {
    static $count = 0;
    echo "defer_cb $count\n";
    $count++;
    if ($count < 10) {
        swoole_event_defer('defer_cb');
    }
}

//swoole_event_defer('defer_cb');
