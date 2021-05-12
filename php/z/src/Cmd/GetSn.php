<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

class GetSn extends Cmd {
    const CMD = Cmd::GetSn;

    function pack() {
        return pack('C', self::CMD);
    }

    static function unpack($bin) {
        $cmd = unpack('C', $bin)[1];
        if ($cmd != self::CMD) return null;
        return new GetSn;
    }
}
