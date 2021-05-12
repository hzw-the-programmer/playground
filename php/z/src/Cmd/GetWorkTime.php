<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

use Hzw\Utils;

class GetWorkTime extends Cmd {
    const CMD = Cmd::GetWorkTime;

    function pack() {
        return pack('C', self::CMD);
    }

    static function unpack($bin) {
        $cmd = unpack('C', $bin)[1];
        if ($cmd !== self::CMD) return null;
        return new GetWorkTime;
    }
}
