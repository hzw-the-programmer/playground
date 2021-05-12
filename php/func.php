<?php
function test() {
    $ret;

    var_dump($ret);
}

test();

class MyClass {
    public $a;
}

$obj = new MyClass();
var_dump($obj->a);
$obj->b;
var_dump($obj->b);
