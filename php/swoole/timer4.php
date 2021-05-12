<?php
$tid = swoole_timer_after(1000, function() {
});
var_dump($tid);
