<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

use Hzw\Utils;

class SetParam extends Cmd {
    const CMD = Cmd::SetParam;

    private $param;

    function __construct($param) {
        $this->param = $param;
    }

    function getParam() {
        return $this->param;
    }

    function pack() {
        $bin = pack('C', self::CMD);
        $bin .= Utils::packParam($this->param);
        return $bin;
    }

    static function unpack($bin) {
        $cmd = unpack('C', $bin)[1];
        if ($cmd !== self::CMD) return null;
        $bin = substr($bin, 1);
        return new self(Utils::unpackParam($bin));
    }
}
