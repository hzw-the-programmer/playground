<?php
use Swoole\Coroutine as co;

$chan = new co\Channel;

co::create(function() use ($chan) {
    echo "1\n";
    $chan->push("hello");
    echo "2\n";
    $chan->push("zhiwenhe");
    echo "4\n";
    sleep(2);
    echo "5\n";
    co::sleep(2);
    echo "8\n";
});

co::create(function() use ($chan) {
    echo "3\n";
    $ret = $chan->pop();
    echo "6 $ret\n";
    $ret = $chan->pop();
    echo "7 $ret\n";
});
