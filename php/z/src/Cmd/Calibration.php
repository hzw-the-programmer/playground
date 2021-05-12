<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

use Hzw\TaskManager;
use Hzw\Task\CalibTask;
use Hzw\Utils;

class Calibration extends Cmd {
    const CMD = Cmd::Calibration;

    private $channels;

    function __construct($channels) {
        $this->channels = $channels;
    }

    function getChannels() {
        return $this->channels;
    }

    function pack() {
        $bin = pack('C', self::CMD);
        $bin .= Utils::packCalibration($this->channels);
        return $bin;
    }

    static function unpack($bin) {
        $cmd = unpack('C', $bin)[1];
        if ($cmd !== self::CMD) return null;
        $bin = substr($bin, 1);
        return new self(Utils::unpackCalibration($bin));
    }

    public function process($cntx, $packet) {
        $header = $packet->getHeader();
        $pcdt = $packet->getPcdt();

        $sns = $header->getSns();
        $dt = $header->getDt();
        $seq = $header->getSeq();

        $sn = $sns[count($sns) - 1];

        TaskManager::normalTask(new CalibTask(
            $sn, $seq, $dt, $pcdt, $this->getChannels()
        ));
    }
}
