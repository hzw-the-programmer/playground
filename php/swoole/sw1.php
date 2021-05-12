<?php
echo "pid=" . getmypid() . "\n";

$fds = [];

$serv = new swoole_server("0.0.0.0", 9501);

$serv->set(["worker_num" => 2, "task_worker_num" => 2]);

$serv->on("Start", function($serv) {
    $pid = getmypid();

    echo "\nonStart: enter\n";
    echo "pid=${pid}\n";
    echo "onStart: exit\n";
});

$serv->on("Shutdown", function($serv) {
    $pid = getmypid();

    echo "\nonShutdown: enter\n";
    echo "pid=${pid}\n";
    echo "onShutdown: exit\n";
});

$serv->on("ManagerStart", function($serv) {
    $pid = getmypid();

    echo "\nonManagerStart: enter\n";
    echo "pid=${pid}\n";
    echo "onManagerStart: exit\n";
});

$serv->on("ManagerStop", function($serv) {
    $pid = getmypid();

    echo "\nonManagerStop: enter\n";
    echo "pid=${pid}\n";
    echo "onManagerStop: exit\n";
});

$serv->on("WorkerStart", function($serv, $worker_id) {
    $pid = $serv->worker_pid;
    $taskworker = $serv->taskworker ? "true" : "false";

    echo "\nonWorkerStart: ${worker_id} enter\n";
    echo "pid=${pid}, worker_id=${worker_id}, taskworker=${taskworker}\n";
    echo "onWorkerStart: ${worker_id} exit\n";
});

$serv->on("WorkerStop", function($serv, $worker_id) {
    $pid = $serv->worker_pid;
    $taskworker = $serv->taskworker ? "true" : "false";

    echo "\nonWorkerStop: ${worker_id} enter\n";
    echo "pid=${pid}, worker_id=${worker_id}, taskworker=${taskworker}\n";
    echo "onWorkerStop: ${worker_id} exit\n";
});

$serv->on("WorkerExit", function($serv, $worker_id) {
    $pid = $serv->worker_pid;
    $taskworker = $serv->taskworker ? "true" : "false";

    echo "\nonWorkerExit: ${worker_id} enter\n";
    echo "pid=${pid}, worker_id=${worker_id}, taskworker=${taskworker}\n";
    echo "onWorkerExit: ${worker_id} exit\n";
});

$serv->on("Connect", function($serv, $fd, $reactor_id) {
    global $fds;
    $pid = $serv->worker_pid;
    $worker_id = $serv->worker_id;
    $taskworker = $serv->taskworker ? "true" : "false";

    if (array_key_exists($fd, $fds)) {
        $fds[$fd]++;
    } else {
        $fds[$fd] = 1;
    }

    echo "\nonConnect: ${worker_id} enter\n";

    echo "pid=${pid}\n";
    echo "work_id=${worker_id}\n";
    echo "reactor_id=${reactor_id}\n";
    echo "taskworker=${taskworker}\n";
    echo "fd=${fd}\n";

    foreach ($fds as $key => $value) {
        echo "${key} ";
    }
    echo "\n";

    foreach ($serv->connections as $_fd) {
        echo "${_fd} ";
    }
    echo "\n";

    foreach ($serv->ports as $port) {
        echo $port->sock . " ";
    }
    echo "\n";

    echo "onConnect: ${worker_id} exit\n";
});

$serv->on("Receive", function($serv, $fd, $reactor_id, $data) {
    global $fds;
    $pid = $serv->worker_pid;
    $worker_id = $serv->worker_id;
    $taskworker = $serv->taskworker ? "true" : "false";

    echo "\nonReceive: ${worker_id} enter\n";

    echo "pid=${pid}\n";
    echo "work_id=${worker_id}\n";
    echo "reactor_id=${reactor_id}\n";
    echo "taskworker=${taskworker}\n";
    echo "fd=${fd}\n";

    foreach ($fds as $key => $value) {
        echo "${key} ";
    }
    echo "\n";

    foreach ($serv->connections as $_fd) {
        echo "${_fd} ";
    }
    echo "\n";

    foreach ($serv->ports as $port) {
        echo $port->sock . " ";
    }
    echo "\n";

    //$fd = 0;
    $serv->after(10000, function() use ($fd) {
        global $fds;
        $pid = getmypid();

        echo "\ntimer: enter\n";
        echo "pid=${pid}\n";
        echo "fd=${fd}\n";
        
        foreach ($fds as $key => $value) {
            echo "${key} ";
        }
        echo "\n";
        
        echo "timer: exit\n";
    });
    $fd = 0;

    $serv->task("zhiwenhe task data.");

    echo "onReceive: ${worker_id} exit\n";
});

$serv->on("Close", function($serv, $fd, $reactor_id) {
    global $fds;
    $pid = $serv->worker_pid;
    $worker_id = $serv->worker_id;
    $taskworker = $serv->taskworker ? "true" : "false";

    unset($fds[$fd]);

    echo "\nonClose: ${worker_id} enter\n";

    echo "pid=${pid}\n";
    echo "work_id=${worker_id}\n";
    echo "reactor_id=${reactor_id}\n";
    echo "taskworker=${taskworker}\n";
    echo "fd=${fd}\n";

    foreach ($fds as $key => $value) {
        echo "${key}";
    }
    echo "\n";

    foreach ($serv->connections as $_fd) {
        echo "${_fd} ";
    }
    echo "\n";

    foreach ($serv->ports as $port) {
        echo $port->sock . " ";
    }
    echo "\n";

    echo "onClose: ${worker_id} exit\n";
});

$serv->on("Task", function($serv, $task_id, $src_worker_id, $data) {
    global $fds;
    $pid = $serv->worker_pid;
    $worker_id = $serv->worker_id;
    $taskworker = $serv->taskworker ? "true" : "false";

    echo "\nonTask: enter\n";
    echo "pid=${pid}\n";
    echo "worker_id=${worker_id}\n";
    echo "taskworker=${taskworker}\n";
    echo "src_worker_id=${src_worker_id}\n";
    echo "task_id={$task_id}\n";
    echo "data=${data}\n";

    foreach ($fds as $key => $value) {
        echo "${key} ";
    }
    echo "\n";

    echo "onTask: exit\n";

    return "zhiwenhe task finish.";
});

$serv->on("Finish", function($serv, $task_id, $data) {
    global $fds;
    $pid = getmypid();

    echo "\nonFinish: enter\n";
    echo "pid=${pid}\n";
    echo "task_id={$task_id}\n";
    echo "data=${data}\n";

    foreach ($fds as $key => $value) {
        echo "${key} ";
    }
    echo "\n";

    echo "onFinish: exit\n";
});

$udp = $serv->addListener("0.0.0.0", 9501, SWOOLE_SOCK_UDP);

$udp->on("Packet", function($serv, $data, $client_info) {
    global $fds;
    $pid = $serv->worker_pid;
    $worker_id = $serv->worker_id;
    $taskworker = $serv->taskworker ? "true" : "false";

    echo "\nonPacket: ${worker_id} enter\n";

    echo "pid=${pid}\n";
    echo "work_id=${worker_id}\n";
    echo "taskworker=${taskworker}\n";

    foreach ($fds as $key => $value) {
        echo "${key} ";
    }
    echo "\n";

    foreach ($serv->connections as $_fd) {
        echo "${_fd} ";
    }
    echo "\n";

    foreach ($serv->ports as $port) {
        echo $port->sock . " ";
    }
    echo "\n";

    echo "onPacket: ${worker_id} exit\n";
});

echo "before start\n";
$serv->start();
echo "after start\n";
