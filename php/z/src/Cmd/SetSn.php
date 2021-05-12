<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

use Hzw\Utils;

class SetSn {
    const CMD = Cmd::SetSn;

    private $sns;

    function __construct($sns) {
        $this->sns = (array)$sns;
    }

    function getSns() {
        return $this->sns;
    }

    function pack() {
        $bin = pack('C', self::CMD);
        $bin .= Utils::packSn($this->sns[0]);
        return $bin;
    }

    static function unpack($bin) {
        $cmd = unpack('C', $bin)[1];
        if ($cmd !== self::CMD) return null;
        $bin = substr($bin, 1);
        return new self([Utils::unpackSn($bin)]);
    }

    function __toString() {
        $str = '{';
        $str .= 'SetSn';
        $str .= ',' . Utils::snsStr($this->sns);
        $str .= '}';
        return $str;
    }
}
