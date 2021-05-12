<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
include "./zprotocal.php";
include "./zutils.php";
include "./zG.php";

class FdInfo {
    public $fd;
    public $id = 0;
    public $tid;

    function __construct($fd) {
        $this->fd = $fd;
    }

    function __destruct() {
        echo "FdInfo({$this->fd}) destruct.\n";
    }
}

class Device {
    public $sn;
    public $ip;
    public $port;
    public $time = 0;

    function __construct($sn) {
        $this->sn = $sn;
    }
}

$serv = new swoole_websocket_server("0.0.0.0", 9998);

$serv->set(["task_worker_num" => 1]);

$serv->on("Task", function($serv, $task_id, $src_worker_id, $data) {
    echo "\nonTask: enter\n";
    dumpWorkerInfo($serv);
    echo "src_worker_id={$src_worker_id}\n";
    echo "task_id={$task_id}\n";
    echo "onTask: exit\n";
});

$serv->on("Finish", function($serv, $task_id, $data) {
    echo "\nonFinish: enter\n";
    dumpWorkerInfo($serv);
    echo "task_id={$task_id}\n";
    echo "onFinish: exit\n";
});

//ps -ef | grep zserver
//swoole_set_process_name("zserver: master");
$serv->on("Start", function($serv) {
    cli_set_process_title("zserver: master");
});

$serv->on("ManagerStart", function($serv) {
    cli_set_process_title("zserver: manager");
});

$serv->on("WorkerStart", function($serv, $worker_id) {
    cli_set_process_title("zserver: worker#{$worker_id}");
    echo "\nonWorkerStart: enter\n";
    dumpWorkerInfo($serv);
    echo "onWorkerStart: exit\n";
});

$serv->on("WorkerStop", function($serv, $worker_id) {
    echo "\nonWorkerStop: enter\n";
    dumpWorkerInfo($serv);
    echo "onWorkerStop: exit\n";
});

$serv->on("Message", function($serv, $frame) {
    echo "\nonMessage: enter\n";
    //dumpWorkerInfo($serv);

    global $g;
    $fd = $frame->fd;
    if (!isset($g->fdInfos[$fd])) {
        $g->fdInfos[$fd] = new FdInfo($fd);
    }
    $fdinfo = $g->fdInfos[$fd];

    if ($fdinfo->id != 0) {
        echo "id != 0\n";
        goto end;
    }
    $fdinfo->id = ++$g->id;

    $ip = NULL;
    $port = 19268;

    $json = json_decode($frame->data);
    if ($json->func == 1 && $json->cmd == 2) {
        $sn = 0;
        $id = $fdinfo->id;
        $dt = time();
        $cmd = Z_CMD_52;
        $data = chr(0x01);

        $ip = $json->ip;
    } else if ($json->func == 2 && $json->cmd == 1) {
        $sn = 79173400000005;
        $id = $fdinfo->id;
        $dt = time();
        $cmd = Z_CMD_ON;
        $data = z_pack_channels($json->channel);

        if (isset($g->devices[$sn])) {
            $ip = $g->devices[$sn]->ip;
            $port = $g->devices[$sn]->port;
        }
    } else if ($json->func == 2 && $json->cmd == 2) {
        $sn = 79173400000005;
        $id = $fdinfo->id;
        $dt = time();
        $cmd = Z_CMD_OFF;
        var_dump($frame->data);
        var_dump($json->channel);
        $data = z_pack_channels($json->channel);

        if (isset($g->devices[$sn])) {
            $ip = $g->devices[$sn]->ip;
            $port = $g->devices[$sn]->port;
        }
    }

    if (!$ip) {
        echo "no ip\n";
        goto end;
    }

    $pkt = z_pack($sn, $id, $dt, $cmd, $data);
    echo bin2hex($pkt) . "\n";

    $serv->sendto($ip, $port, $pkt);
    
    $fdinfo->tid = $serv->after(5000, function() use ($serv, $fd) {
        echo "\ntimer: enter\n";
        //dumpWorkerInfo($serv);

        global $g;
        if (isset($g->fdInfos[$fd])) {
            $fdinfo = $g->fdInfos[$fd];
            $fdinfo->id = 0;
            $serv->push($fdinfo->fd, json_encode(["result" => 0]));
        }
        echo "timer: exit\n";
    });

    end:
        echo "onMessage: exit\n";
});

$serv->on("Close", function($serv, $fd) {
    echo "\nonClose: enter\n";
    dumpWorkerInfo($serv);

    global $g;
    unset($g->fdInfos[$fd]);

    echo "onClose: exit\n";
});

$udp = $serv->listen("0.0.0.0", 19268, SWOOLE_SOCK_UDP);

$udp->on("Packet", function($serv, $data, $clientinfo) {
    //echo "\nonPacket: enter\n";
    //dumpWorkerInfo($serv);

    $time = time();
    echo "R " . date("Y-m-d H:i:s", $time) . " " . bin2hex($data) . "\n";

    list($valid, $ver, $len, $sn, $id, $dt, $cmd, $data) = z_unpack($data);
    //echo "valid: " . ($valid ? "TRUE\n" : "FALSE\n");
    if (!$valid) goto end;
    /*
    echo "ver:   ${ver}\n";
    echo "len:   ${len}\n";
    echo "sn:    ${sn}\n";
    echo "id:    ${id}\n";
    echo "dt:    " . date("Y-m-d H:i:s", $dt) . "\n";
    echo "cmd:   0x" . dechex($cmd) . "\n";
    echo "data:  " . bin2hex($data) . "\n";
    */
    
    global $g;
    if (!isset($g->devices[$sn])) {
        $g->devices[$sn] = new Device($sn);
    }
    $device = $g->devices[$sn];
    $device->ip = $clientinfo["address"];
    $device->port = $clientinfo["port"];
    if ($device->time == 0) {
        echo date("Y-m-d H:i:s", $time) . " " . "{$device->ip}:{$device->port} online\n";
    }
    $device->time = $time;
    $g->offlineDetect($device);

    if ($cmd == Z_CMD_ACK) {
        $ack = z_unpack_id($data)[0];
        foreach ($g->fdInfos as $fd => $fdinfo) {
            if ($fdinfo->id == $ack) {
                $fdinfo->id = 0;
                $serv->clearTimer($fdinfo->tid);
                $serv->push($fdinfo->fd, json_encode(["result" => 1, "sn" => $sn]));
                break;
            }
        }
    } else {
        $pkt = z_pack($sn, 0, time(), Z_CMD_ACK, z_pack_id($id));
        //echo bin2hex($pkt) . "\n";
        $serv->sendto($device->ip, $device->port, $pkt);
    }

    end:
        //echo "onPacket: exit\n";
});

$g = new G($serv);

$serv->start();
