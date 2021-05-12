<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

class LoraUtils {
    public static function pack($data) {
        return "\x0a\x01\x02" . pack('n', strlen($data) + 1) . $data . "\x00";
    }

    public static function unpack($data) {
        return substr($data, 5, -1);
    }
}
