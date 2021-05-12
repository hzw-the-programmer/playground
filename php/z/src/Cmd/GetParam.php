<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

class GetParam {
    const CMD = Cmd::GetParam;

    private $type;

    function __construct($type) {
        $this->type = $type;
    }

    function getType() {
        return $this->type;
    }

    function pack() {
        $bin = pack('C', self::CMD);
        $bin .= pack('C', $this->type);
        return $bin;
    }

    static function unpack($bin) {
        $cmd = unpack('C', $bin)[1];
        if ($cmd != self::CMD) return null;
        $bin = substr($bin, 1);
        return new self(unpack('C', $bin)[1]);
    }

    function __toString()
    {
        $str = '{';
        $str .= 'GetParam';
        $str .= ",{$this->type}";
        $str .= '}';
        return $str;
    }
}
