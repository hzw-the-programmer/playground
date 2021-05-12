<?php
$fn = '/tmp/touch';
$at = @fileatime($fn);
if ($at !== false) {
    echo date('Y-m-d H:i:s', $at) . PHP_EOL;
}

swoole_timer_tick(1000, function() {
    touch('/tmp/touch');
});
