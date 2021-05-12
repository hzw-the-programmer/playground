<?php
echo date('Y-m-d H:i:s') . PHP_EOL;
swoole_timer_tick(1000, function() {
    echo date('Y-m-d H:i:s') . PHP_EOL;
});

sleep(10);
