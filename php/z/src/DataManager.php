<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

class DataManager {
    static function filename($id) {
        return Config::DATA_DIR . "/data$id";
    }

    static function backup($fn, $cb) {
        $f = fopen($fn, 'a');
        $cb($f);
        fclose($f);
    }

    static function restore($fn, $cb) {
        clearstatcache();
    
        if (!is_file($fn)) {
            return true;
        }
    
        $size = filesize($fn);
        if (!$size) {
            return true;
        }
    
        $mfn = "$fn.meta";
        if (is_file($mfn)) {
            $pos = file_get_contents($mfn);
        } else {
            $pos = 0;
        }
    
        $f = fopen($fn, 'r+');
        fseek($f, $pos);
    
        $cb($f);
    
        $pos = ftell($f);
        if ($pos === $size) {
            ftruncate($f, 0);
            $pos = 0;
        }
    
        fclose($f);
    
        file_put_contents($mfn, sprintf("%d", $pos));
    
        return $pos === 0;
    }    
}
