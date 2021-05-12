<?php
$pid = getmypid();

function sig_handler($signo) {
    echo "signo=$signo\n";
}

declare(ticks = 1);
pcntl_signal(SIGTERM, "sig_handler");

$lock = new swoole_lock(SWOOLE_MUTEX);
echo "[$pid] lock\n";
$lock->lock();
echo "[$pid] lock success\n";

switch (pcntl_fork()) {
    case 0:
        $pid = getmypid();
        echo "[$pid] lock\n";
        $lock->lock();
        echo "[$pid] lock success\n";
        sleep(10);
        echo "[$pid] unlock\n";
        $lock->unlock();
        echo "[$pid] unlock success\n";
        exit();
        //exit(1);
        break;
    case -1:
        echo "[Master] fork failed\n";
        break;
    default:
        sleep(5);
        echo "[$pid] unlock\n";
        $lock->unlock();
        echo "[$pid] unlock success\n";
        break;
}

$cpid = pcntl_wait($status);
echo "[Child($cpid)] exit\n";
echo "status: $status\n";
echo "exited: " . (pcntl_wifexited($status) ? "TRUE" : "FALSE") . "\n";
echo "exitstatus: " . pcntl_wexitstatus($status) . "\n";
echo "signaled: " . (pcntl_wifsignaled($status) ? "TRUE" : "FALSE") . "\n";
echo "signal: " . pcntl_wtermsig($status) . "\n";
echo "stopped: " . (pcntl_wifstopped($status) ? "TRUE" : "FALSE") . "\n";
sleep(20);
echo "[$pid] finish\n";
