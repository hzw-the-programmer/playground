<?php
class A {
    const NAME = 'A';

    function f() {
        echo 'called class: ' . get_called_class() . PHP_EOL;
        echo 'self: ' . self::NAME . PHP_EOL;
        echo 'static: ' . static::NAME . PHP_EOL;
    }
}

class B extends A {
    const NAME = 'B';
}

$b = new B;
$b->f();
