<?php
namespace Hzw;

class p {
    function tf() {
        echo "self::class:\n";
        var_dump(self::class);
        $pos = strrpos(self::class, '\\');
        if ($pos === false) {
            echo self::class . "\n";
        } else {
            echo substr(self::class, $pos + 1) . "\n";
        }
        echo "get_class:\n";
        var_dump(get_class());
        echo "__CLASS__:\n";
        var_dump(__CLASS__);
        
        echo "static::class:\n";
        var_dump(static::class);
        echo "get_called_class:\n";
        var_dump(get_called_class());
    }
}

class c extends p {

}

$o = new P;
$o->tf();
echo "\n";
$o = new c;
$o->tf();
