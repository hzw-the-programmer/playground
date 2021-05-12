<?php
class G {
    public $id = 0;
    public $fdInfos = [];
    public $devices = [];
    public $tid = 0;
    public $serv;

    function __construct($serv) {
        $this->serv = $serv;
    }

    public function nextDevice() {
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

    public function calTime($time) {
        $time += 15;
        return ($time - time()) * 1000;
    }

    public function timercb($time, $device) {
        $this->tid = 0;
        if ($time == $device->time) {
            $device->time = 0;
            echo date("Y-m-d H:i:s") . " " . "{$device->ip}:{$device->port} offline\n";
        }
        $device = $this->nextDevice();
        if ($device != NULL) {
            $this->offlineDetect($device);
        }
    }

    public function offlineDetect($device) {
        $time = $device->time;
        if ($this->tid == 0) {
            $ms = $this->calTime($time);
            $this->tid = $this->serv->after($ms, function() use ($time, $device) {
                $this->timercb($time, $device);
            });
        }
    }
}
