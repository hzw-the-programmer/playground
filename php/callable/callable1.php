<?php
function func1($a, $b) {
    echo "$a $b\n";
}
$func = 'func1';
$func('call func', 'as a string variable');

class O {
    static function sm($msg) {
        echo "static::sm: $msg\n";
    }

    function m1($msg) {
        echo "instance::m1: $msg\n";
    }

    function m2($msg) {
        echo "instance::m2: $msg\n";
    }
}

$func = 'O::sm';
$func('call static method as a string variable');
$func = ['O', 'sm'];
$func('call static method as an array of strings');

$o = new O();
$func = [$o, 'm1'];
$func('call instance method as an array');

$m = 'm1';
$o->$m('haha');
$m = 'm2';
$o->$m('hehe');
