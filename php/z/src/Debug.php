<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

class Debug {
    static $cntx;

    static function init($cntx) {
        static::$cntx = $cntx;
    }
    
    static function measure($cb) {
        $start = microtime(true);
        $result = $cb();
        $elapsed = microtime(true) - $start;
        return [$result, $elapsed];
    }

    static function measureAndLog($cb, $msg) {
        $logger = static::$cntx->getLogger();

        $start = microtime(true);
        $result = $cb();
        $elapsed = microtime(true) - $start;

        $logger->debug(sprintf($msg, $elapsed));

        return $result;
    }
}
