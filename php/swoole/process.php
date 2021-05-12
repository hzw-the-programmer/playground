<?php
echo getmypid() . "\n";

swoole_set_process_name("swoole: master");

swoole_process::signal(SIGCHLD, function($signo) {
    $ret = swoole_process::wait(false);
    var_dump($ret);
});

$process = new swoole_process("worker");
$process->start();

function worker($process) {
    swoole_set_process_name("swoole: worker");

    echo "\nworker enter\n";
    echo getmypid() . "\n";
    sleep(10);
    echo "worker exit\n";
}
