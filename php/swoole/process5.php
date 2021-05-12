<?php
use \Swoole\Process;

$p = new Process(function($p) {
    $msg = "haha\n";
    echo $msg;
    $p->write($msg);
});

var_dump($p->callback);

$p->callback = function($p) {
    $msg = "hehe\n";
    echo "echo $msg";
    $p->write("write $msg");
};

echo "before start\n";
var_dump($p->pid);
var_dump($p->pipe);

$pid = $p->start();

echo "after start $pid\n";
var_dump($p->pid);
var_dump($p->pipe);

echo $p->read() . PHP_EOL;
