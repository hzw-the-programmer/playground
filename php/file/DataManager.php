<?php
class DataManager {
    static function backup($fn, $cb) {
        $f = fopen($fn, 'a');
        $cb($f);
        fclose($f);
    }

    static function restore($fn, $cb) {
        clearstatcache();
    
        if (!is_file($fn)) {
            return false;
        }
    
        $size = filesize($fn);
        if (!$size) {
            return false;
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
    
        return $pos !== 0;
    }    
}
