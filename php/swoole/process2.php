<?php
swoole_process::signal(SIGCHLD, function($sig) {
    echo swoole_process::wait(false)["pid"] . "\n";
});

$child = new Swoole\Process(function($process) {
    $process->close(1); //read side
    $bytes = 0;
    for ($i = 0; $i < 10; $i++) {
        $data = str_repeat("A", 100);
        $bytes += $process->write($data);
    }
    $process->close(2); //write side
    echo getmypid() . ": write $bytes\n";
});
$child->start();

$child->close(2); //write side
swoole_event_add($child->pipe, function($pipe) use ($child) {
    static $bytes = 0;
    $data = $child->read();
    echo getmypid() . ": read $data\n";
    $bytes += strlen($data);
    echo getmypid() . ": read $bytes\n";
});
