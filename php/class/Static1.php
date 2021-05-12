<?php

class A {
    static $name = "A";

    static function init($cntx) {
        echo static::$name . PHP_EOL;
    }
}

class B extends A {
    static $name = "B";
    static function init($cntx) {
        echo static::$name . PHP_EOL;
        //forward_static_call(["A", "init"], $cntx);
        A::init(null);
    }
}

A::init(null);
B::init(null);
