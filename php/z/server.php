<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
include_once "./config.php";
include_once "./logger.php";
include_once "./zprotocal.php";
include_once "./device.php";
include_once "./db.php";
include_once "../db_config.php";

class Server {
    private $serv;
    private $logger;
    private $devices;
    private $offlineTimer;

    public function __construct() {
        $serv = $this->serv = new swoole_websocket_server("0.0.0.0", 9998);
        $udp = $serv->listen("0.0.0.0", 19268, SWOOLE_SOCK_UDP);

        $serv->set(["task_worker_num" => 1]);

        $serv->on("Start", [$this, "onStart"]);
        $serv->on("ManagerStart", [$this, "onManagerStart"]);
        $serv->on("WorkerStart", [$this, "onWorkerStart"]);

        $serv->on("Message", [$this, "onMessage"]);
        $udp->on("Packet", [$this, "onPacket"]);
        $serv->on("Task", [$this, "onTask"]);
        $serv->on("Finish", [$this, "onFinish"]);

        $this->logger = new Logger();
        $this->db = new Db(DSN, USERNAME, PASSWD);

        $this->createDirs();
    }

    public function run() {
        $this->serv->start();
    }

    private function createDirs() {
        if (!file_exists(PID_DIR)) {
            mkdir(PID_DIR);
        }
        if (!file_exists(LOG_DIR)) {
            mkdir(LOG_DIR);
        }
    }

    private function updateDevice($sn, $clientInfo, $time) {
        if (!isset($this->devices[$sn])) {
            $this->devices[$sn] = new Device($sn);
        }

        $device = $this->devices[$sn];
        $device->ip = $clientInfo["address"];
        $device->port = $clientInfo["port"];
        if ($device->time == 0) {
            echo date("Y-m-d H:i:s ", $time) . $device->sn . " online\n";
        }
        $device->time = $time;
    }

    private function getOldestDevice() {
        $result = NULL;

        foreach ($this->devices as $device) {
            if ($device->time == 0) {
                continue;
            }
            if ($result == NULL || $result->time > $device->time) {
                $result = $device;
            }
        }

        return $result;
    }

    private function offlineDetect() {
        if ($this->offlineTimer) return;

        while ($device = $this->getOldestDevice()) {
            $time = time();
            $timeout = $device->time + OFFLINE_TIMEOUT - $time;

            if ($timeout > 0) {
                $this->offlineTimer = $this->serv->after($timeout * 1000, function() {
                    $this->offlineTimer = NULL;
                    $this->offlineDetect();
                });
                break;
            }

            $device->time = 0;
            echo date("Y-m-d H:i:s ", $time) . $device->sn . " offline\n";
        }    
    }

    public function onStart($serv) {
        swoole_set_process_name("zserver: master");
        file_put_contents(PID_DIR . "/pid", posix_getpid());
    }

    public function onManagerStart($serv) {
        swoole_set_process_name("zserver: manager");
    }

    public function onWorkerStart($serv, $workerId) {
        swoole_set_process_name("zserver: worker#{$workerId}");
    }

    public function onMessage($serv, $frame) {

    }

    public function onPacket($serv, $data, $clientInfo) {
        $time = time();
        
        $this->logger->log($time, "R", $clientInfo, $data);

        list($valid, $ver, $len, $sn, $id, $dt, $cmd, $remain) = z_unpack($data);
        if (!$valid) return;
        if ($sn < 10000000000000 || $sn > 99999999999999) return;

        $this->updateDevice($sn, $clientInfo, $time);
        $this->offlineDetect();

        switch ($cmd) {
            case Z_CMD_ACK:
                return;
            
            case Z_CMD_DATA:
            case Z_CMD_STATUS:
                $serv->task($data);
                break;
        }

        $data = z_pack($sn, 0, $time, Z_CMD_ACK, z_pack_id($id));
        $serv->sendto($clientInfo["address"], $clientInfo["port"], $data);
        $this->logger->log($time, "W", $clientInfo, $data);
    }

    public function onTask($serv, $taskId, $srcWorkerId, $data) {
        list($valid, $ver, $len, $sn, $id, $dt, $cmd, $remain) = z_unpack($data);

        switch ($cmd) {
            case Z_CMD_DATA:
                $this->db->newChannelsData($sn, $dt, $remain);
                break;
            case Z_CMD_STATUS:
                $this->db->newChannelsStatus($sn, $dt, $remain);
                break;
        }
    }

    public function onFinish($serv, $taskId, $data) {

    }
}
