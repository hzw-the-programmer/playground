<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

use Hzw\Utils;

class SetWorkTime extends Cmd {
    const CMD = Cmd::SetWorkTime;
    
    private $periods;

    function __construct($periods) {
        $this->periods = $periods;
    }

    function getPeriods() {
        return $this->periods;
    }

    function pack() {
        $bin = pack('C', self::CMD);
        $bin .= Utils::packPeriods($this->periods);
        return $bin;
    }

    static function unpack($bin) {
        $cmd = unpack('C', $bin)[1];
        if ($cmd !== self::CMD) return null;
        $bin = substr($bin, 1);
        return new self(Utils::unpackPeriods($bin));
    }
}
