<?php
echo getmypid() . "\n";

class MyClass {
    function __destruct() {
        echo "\ndestruct\n";
    }
}

function timeout_cb($timerid, $data) {
    static $state = 1;

    echo "\n" . getmypid() . "\n";
    var_dump($data);
    $data->pro++;
    var_dump($state);
    var_dump($timerid);
    if ($state == 5) {
        swoole_timer_clear($timerid);
    }
    $state++;
}

//$data = 1;
$data = new MyClass();
$data->pro = 1;
swoole_timer_tick(1000, 'timeout_cb', $data);
$data->pro = 2;
//$data = 2;

echo "begin\n";
