<?php
echo getmypid() . "\n";

class MyClass {
    function __destruct() {
        echo "\ndestruct\n";
    }
}

/*
$int = 1;
$timer1 = swoole_timer_after(10000, function($int) {
    echo "\nIn Timer Callback\n";
    echo getmypid() . "\n";
    echo "\$int=$int\n";
}, $int);
$int = 2;
*/

/*
$obj = new MyClass();
$obj->int = 1;
$timer1 = swoole_timer_after(10000, function($obj) {
    echo "\nIn Timer Callback\n";
    echo getmypid() . "\n";
    echo "\$obj->int={$obj->int}\n";
}, $obj);
$obj->int = 2;
*/

$int = 1;
$obj = new MyClass();
$obj->int = 1;
$timer1 = swoole_timer_after(10000, function() use ($int, $obj) {
    echo "\nIn Timer Callback\n";
    echo getmypid() . "\n";
    echo "\$int=$int\n";
    echo "\$obj->int={$obj->int}\n";
});
$int = 2;
$obj->int = 2;

var_dump($timer1);

//swoole_process::signal(SIGTERM, function() {
//    swoole_event_exit();
//});
