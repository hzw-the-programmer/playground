<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

class HeartBeat extends Cmd {
    const CMD = Cmd::HeartBeat;

    private $signal;

    function __construct($signal) {
        $this->signal = $signal;
    }

    function getSignal() {
        return $this->signal;
    }

    function pack() {
        $bin = pack('C', self::CMD);
        $bin .= pack('c', $this->signal);
        return $bin;
    }

    static function unpack($bin) {
        $cmd = unpack('C', $bin)[1];
        if ($cmd !== self::CMD) return null;
        $bin = substr($bin, 1);
        return new self(unpack('c', $bin)[1]);
    }
}
