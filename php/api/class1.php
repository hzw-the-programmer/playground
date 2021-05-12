<?php
class A {
    public static function task() {
        return static::class;
    }

    public static function onFinish() {
        return __METHOD__;
    }
}

class B extends A {
}

var_dump(A::task()::onFinish());
var_dump(B::task());
