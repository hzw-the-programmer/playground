<?php
class A {
    function f() {
        $this->f1();
        static::f1();
    }

    function f1() {
        echo __METHOD__ . PHP_EOL;
    }
}

class B extends A {
    function f1() {
        echo __METHOD__ . PHP_EOL;
    }
}

$b = new B;
$b->f();
