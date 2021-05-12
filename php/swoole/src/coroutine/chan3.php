<?php
use Swoole\Coroutine as co;

$chan = new co\Channel(3);

co::create(function() use ($chan) {
    echo "1\n";
    $chan->push("hello");
    echo "2\n";
    $chan->push("zhiwenhe");
    echo "3\n";
    $chan->push("my sweet heart.");
    echo "4\n";
    $chan->push("interesting");
    echo "6\n";
});

co::create(function() use ($chan) {
    echo "5\n";
    $ret = $chan->pop();
    echo "7 $ret\n";
    $ret = $chan->pop();
    echo "8 $ret\n";
    $ret = $chan->pop();
    echo "9 $ret\n";
});
