<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

class PowerOn extends Cmd {
    const CMD = Cmd::PowerOn;

    function pack() {
        return pack('C', self::CMD);
    }

    static function unpack($bin) {
        $cmd = unpack('C', $bin)[1];
        if ($cmd !== self::CMD) return null;
        return new self();
    }
}
