<?php
$serv = new swoole_websockt_server('0.0.0.0', 9998);
$serv->set(['worker_num' => 2]);

$serv->on('Start', function($serv) {
    echo "onStart: pid=" . getmypid() . ", master_pid=" . $serv->master_pid . ", manager_pid=" . $serv->manager_pid . "\n";
});

$serv->on('Shutdown', function($serv) {
    echo "onShutdown: pid=" . getmypid() . "\n";
});

$serv->on('WorkerStart', function($serv, $worker_id) {
    echo "onWorkerStart: pid=" . getmypid() . ", worker_id=${worker_id}\n";
});

$serv->on('WorkerStop', function($serv, $worker_id) {
    echo "onWorkerStop: pid=" . getmypid() . ", worker_id=${worker_id}\n";
});

$serv->on('WorkerExit', function($serv, $worker_id) {
    echo "onWorkerExit: pid=" . getmypid() . ", worker_id=${worker_id}\n";
});

$serv->on('Connect', function(swoole_server $serv, int $fd, int $reactorId) {
    echo "onConnect: pid=" . getmypid() . ", fd=${fd}, reactorId={$reactorId}\n";
});

$serv->on('Receive', function(swoole_server $serv, int $fd, int $reactorId, string $data) {
    echo "onReceive: pid=" . getmypid() . ", fd=${fd}, reactorId={$reactorId}\n";
    var_dump($data);
});

$serv->on('Message', function($serv, $frame) {
    echo "onMessage: pid=" . getmypid() . "\n";
});

$serv->start();

/*
$udp = $ws->listen('0.0.0.0', 19268, SWOOLE_SOCK_UDP);

$ws->on('message', function($server, $frame) use ($udp) {
    $data = json_decode($frame->data);
    $pkt = "\x50\x33" .
           "\x00\x15" .
           "\x00\x00\x00\x00\x00\x00\x00\x00" .
           "\x00\x00\x00\x01" .
           "\x12\x03\x17\x10\x1C\x14" .
           "\x52\x01" .
           "\x4C";
    $udp->sendto($data->ip, 19268, $pkt);
});

$udp->on('packet', function($serv, $data, $addr) {
    var_dump($data);
    var_dump($addr);
});
*/
