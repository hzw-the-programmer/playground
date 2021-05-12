<?php
use Swoole\Coroutine as co;

co::create(function() {
    echo "1\n";
    $redis = new co\Redis();
    echo "2\n";
    $redis->connect('127.0.0.1', 6379);
    echo "4\n";
    co::create(function() use ($redis) {
        echo "5\n";
        $redis->set('foo', 'bar');
        echo "6\n";
        co::create(function() use ($redis) {
            echo "7\n";
            $res = $redis->get('foo');
            echo "8 $res\n";
        });
        echo "9\n";
    });
    echo "10\n";
});

echo "3\n";
