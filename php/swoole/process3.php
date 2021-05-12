<?php
$mpid = getmypid();
echo "master: mpid=$mpid\n";

$process = new swoole_process('do_work', true);
//$process = new swoole_process('do_work', true, 2);
//$process = new swoole_process('do_work');
//$process = new swoole_process('do_work', false, 1);
$cpid = $process->start();
echo "master: cpid=$cpid\n";

sleep(10);
echo 'master: read: ' . $process->read() . "\n";
//echo 'master: read: ' . $process->read() . "\n";

function do_work($process) {
    $pid = getmypid();
    echo "child: pid=$pid sleep 3s.";
    sleep(3);
    echo 'child: pid=' . getmypid();
    $process->write('child: hehe');
    $process->write('child: haha');
}
