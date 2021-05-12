<?php
$t = time();
echo date('Y-m-d H:i:s', $t) . PHP_EOL ;

$d = 60 - $t % 60;
echo "$d\n";
echo date('Y-m-d H:i:s', $t + $d) . PHP_EOL ;

swoole_timer_after($d * 1000, function() {
    echo date('Y-m-d H:i:s') . PHP_EOL ;
});
