<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

use Hzw\TaskManager;
use Hzw\Task\DeviceStandbyHBTask;

class DeviceStandbyHB extends Cmd {
    const CMD = Cmd::DeviceStandbyHB;

    public function getSignal() {
        return 1;
    }

    public function pack() {
        $bin = pack('C', self::CMD);
        return $bin;
    }

    public static function unpack($bin) {
        $cmd = unpack('C', $bin)[1];
        if ($cmd !== self::CMD) return null;
        return new self();
    }

    public function process($cntx, $packet) {
        $header = $packet->getHeader();
        $pcdt = $packet->getPcdt();

        $sns = $header->getSns();
        $dt = $header->getDt();

        $sn = $sns[count($sns) - 1];

        $signal = $this->getSignal();
        if ($extraInfo = $packet->getExtraInfo()) {
            $signal = $extraInfo->rssi;
        }

        TaskManager::realtimeTask(new DeviceStandbyHBTask(
            $sn, $dt, $signal
        ));
    }
}
