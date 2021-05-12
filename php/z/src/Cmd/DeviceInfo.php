<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

use Hzw\Utils;

class DeviceInfo extends Cmd {
    const CMD = Cmd::DeviceInfo;

    private $version;

    public function __construct($version) {
        $this->version = $version;
    }

    public function getVersion() {
        return $this->version;
    }

    public function pack() {
        $bin = pack('C', self::CMD);
        $bin .= Utils::packVersion($this->version);
        return $bin;
    }

    public static function unpack($bin) {
        $cmd = unpack('C', $bin)[1];
        if ($cmd !== self::CMD) return null;
        $bin = substr($bin, 1);
        return new self(Utils::unpackVersion($bin));
    }
}
