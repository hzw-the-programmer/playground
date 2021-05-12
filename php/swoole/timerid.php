<?php
$tid = swoole_timer_after(1000, function () {
    echo "after callback no argument.\n";
});

echo $tid . "\n";
