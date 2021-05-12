<?php
function dumpWorkerInfo($serv) {
    $worker_pid = $serv->worker_pid;
    $worker_id = $serv->worker_id;
    $taskworker = $serv->taskworker ? "True" : "False";

    echo "worker_pid={$worker_pid}\n";
    echo "worker_id={$worker_id}\n";
    echo "taskworker={$taskworker}\n";
}
