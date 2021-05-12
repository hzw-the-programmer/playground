<?php
class A {
    const NAME = 'A';

    static function f1() {
        echo get_called_class() . ' ' . static::NAME . PHP_EOL;
    }
}

class B extends A {
    const NAME = 'B';

    static function f() {
        B::f1();
        A::f1();
        forward_static_call(['A', 'f1']);
    }
}

B::f();
