<?php
use Swoole\Coroutine as co;
$chan = new co\Channel();

co::create(function() use ($chan) {
    $cid = co::getuid();
    echo "1: co$cid sleep 1 second.\n";
    co::sleep(1);
    echo "3: co$cid push.\n";
    if ($chan->push('hello zhiwenhe.') === true) {
        echo "4: co$cid push ok.\n";
    }
    echo "5: co$cid finish.\n";
});

co::create(function() use ($chan) {
    $cid = co::getuid();
    echo "2: co$cid pop.\n";
    $ret = $chan->pop();
    echo "6: co$cid get $ret\n";
    sleep(2);
});
