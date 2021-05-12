<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

use Hzw\Utils;

class PowerOff extends Cmd {
    const CMD = Cmd::PowerOff;

    private $dt;

    function __construct($dt) {
        $this->dt = $dt;
    }

    function getDt() {
        return $this->dt;
    }

    function pack() {
        return pack('C', self::CMD);
    }

    static function unpack($bin) {
        $cmd = unpack('C', $bin)[1];
        if ($cmd != self::CMD) return null;
        $bin = substr($bin, 1);
        return new self(Utils::unpackDt($bin));
    }
}
