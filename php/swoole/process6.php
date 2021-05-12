<?php
use \Swoole\Process;

echo sprintf("before daemon pid is %d\n", getmypid());
Process::daemon();
echo sprintf("after daemon pid is %d\n", getmypid());

swoole_set_process_name(sprintf("%s: master", $argv[0]));

$count = 3;

$children = [];
for ($i = 0; $i < $count; $i++) {
    $child = createChild($i);
    if ($child) {
        $children[$child->pid] = $child;
    }
}

Process::signal(SIGCHLD, function($signo) use (&$children) {
    while ($ret = Process::wait(false)) {
        $pid = $ret['pid'];
        echo sprintf("process %d exit, code=%d, signal=%d\n", $pid, $ret['code'], $ret['signal']);
        if (isset($children[$pid])) {
            $id = $children[$pid]->id;
            unset($children[$pid]);
            $child = createChild($id);
            if ($child) {
                $children[$child->pid] = $child;
            }
        }
    }
});

function createChild($id) {
    $child = new Process('child');
    $child->id = $id;
    $pid = $child->start();
    if ($pid === false) {
        $errno = swoole_errno();
        echo sprintf("%d(%s)\n", $errno, swoole_strerror($errno));
        return null;
    }
    return $child;
}

function child($child) {
    global $argv;
    swoole_set_process_name(sprintf("%s: child %d", $argv[0], $child->id));
    while (true) {}
}
