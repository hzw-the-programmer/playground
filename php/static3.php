<?php
class A {
    static function newSelf() {
        return new self;
    }

    static function newStatic() {
        return new static;
    }
}

class B extends A {
}

echo get_class(A::newSelf()) . PHP_EOL;
echo get_class(A::newStatic()) . PHP_EOL;
echo get_class(B::newSelf()) . PHP_EOL;
echo get_class(B::newStatic()) . PHP_EOL;
