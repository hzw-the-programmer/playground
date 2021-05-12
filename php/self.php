<?php
class A {
    static function main() {
        echo __CLASS__ . "\n";
    }

    static function test() {
        //self::main();
        static::main();
    }
}

class B extends A {
    static function main() {
        echo __CLASS__ . "\n";
    }
}

B::test();
