<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

use Hzw\Utils;

class Data extends Cmd {
    const CMD = Cmd::Data;

    private $channels;

    function __construct($channels) {
        $this->channels = $channels;
    }

    function getChannels() {
        return $this->channels;
    }

    function pack() {
        $bin = pack('C', self::CMD);
        $bin .= Utils::packData($this->channels);
        return $bin;
    }

    static function unpack($bin) {
        $cmd = unpack('C', $bin)[1];
        if ($cmd !== self::CMD) return null;
        $bin = substr($bin, 1);
        return new self(Utils::unpackData($bin));
    }
}
