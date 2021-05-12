<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

use Hzw\Utils;

class Open extends Cmd {
    const CMD = Cmd::Open;

    private $channels;

    function __construct($channels) {
        $this->channels = $channels;
    }

    function getChannels() {
        return $this->channels;
    }

    function pack() {
        $bin = pack('C', static::CMD);
        $bin .= Utils::packChannels($this->channels);
        return $bin;
    }

    static function unpack($bin) {
        $cmd = unpack('C', $bin)[1];
        if ($cmd !== static::CMD) return null;
        $bin = substr($bin, 1);
        return new static(Utils::unpackChannels($bin));
    }
}
