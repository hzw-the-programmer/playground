<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

use Hzw\Task\DeviceStatus;
use Hzw\Task\NewDevice;
use Hzw\Task\UpdateDevice;
use Hzw\Config;

class DeviceManager {
    private $cntx;
    private $devices;
    private $tid;

    function __construct($cntx) {
        $this->cntx = $cntx;
        $this->devices = [];
        $this->tid = 0;
    }

    function setDevices($devices) {
        $this->devices = $devices;
    }

    function getDevice($sn) {
        if (isset($this->devices[$sn])) {
            return $this->devices[$sn];
        }
        return null;
    }

    function addDevice($device) {
        $this->devices[$device->getSn()] = $device;
        return $this;
    }

    function onPacket($packet) {
        touch(Config::LOG_DIR . '/touch');

        $header = $packet->getHeader();
        $source = $packet->getSource();
        $pcdt = $packet->getPcdt();

        $sns = $header->getSns();

        $device = $this->updateDevice($sns, $source, $pcdt);
        if ($device->getPcdt() === 0) {
            TaskManager::realtimeTask(new DeviceStatus(
                $device->getSn(), 0, $pcdt, $pcdt, Device::ONLINE
            ));
        }
        $device->setPcdt($pcdt);

        $this->offlineDetect($pcdt);
    }

    private function offlineDetect($pcdt) {
        $serv = $this->cntx->getServ();

        if ($this->tid) return;

        $device = $this->getOldestDevice();
        if (!$device) return;

        $chkdt = $device->getPcdt() + Config::OFFLINE_TIMEOUT;
        $timeout = $chkdt - $pcdt;
        if ($timeout > 0) {
            $this->tid = $serv->after($timeout * 1000, function() {
                $this->tid = 0;
                $this->offlineDetect(time());
            });
        } else {
            TaskManager::realtimeTask(new DeviceStatus(
                $device->getSn(), 0, $pcdt, $device->getPcdt(), Device::OFFLINE
            ));
            $device->setPcdt(0);
            $this->offlineDetect($pcdt);
        }
    }

    private function getOldestDevice() {
        $result = null;

        foreach ($this->devices as $device) {
            $pcdt = $device->getPcdt();

            if ($pcdt == 0) {
                continue;
            }

            if ($result == NULL || $result->getPcdt() > $pcdt) {
                $result = $device;
            }
        }

        return $result;
    }

    function updateDevice($sns, $address, $dt) {
        $psn = '';
        $device = null;
        foreach ($sns as $sn) {
            $device = $this->getDevice($sn);
            if (!$device) {
                $device = new Device($sn);
                $this->addDevice($device);
            }
            if ($device->getPsn() !== $psn
                || $device->getAddress() !== $address
            ) {
                $device->setPsn($psn)
                       ->setAddress($address);
                TaskManager::normalTask(new UpdateDevice(
                    $sn, $psn, $address, $dt
                ));
            }
            $psn = $sn;
        }
        return $device;
    }
}
