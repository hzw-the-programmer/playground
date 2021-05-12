<?php
$workerNum = 10;

$pool = new Swoole\Process\Pool($workerNum);

$pool->on('WorkerStart', function($pool, $workerId) {
    $pid = getmypid();
    echo "worker $workerId is started: $pid\n";
    swoole_set_process_name("pool1 worker $workerId");
    while (true) {}
});

$pool->on('WorkerStop', function($pool, $workerId) {
    $pid = getmypid();
    echo "worker $workerId is stopped: $pid\n";
});

$pool->start();
