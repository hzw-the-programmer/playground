<?php
class A {
    function f() {
        $this->f1();
    }

    function g() {
        self::g1();
    }

    function h() {
        static::h1();
    }

    function j() {
        $this->j1();
    }

    function k() {
        $this->k1();
    }

    function f1() {
        echo __METHOD__ . PHP_EOL;
    }

    function g1() {
        echo __METHOD__ . PHP_EOL;
    }

    function h1() {
        echo __METHOD__ . PHP_EOL;
    }

    protected function j1() {
        echo __METHOD__ . PHP_EOL;
    }

    private function k1() {
        echo __METHOD__ . PHP_EOL;
    }
}

class B extends A {
    function f1() {
        echo __METHOD__ . PHP_EOL;
    }

    function g1() {
        echo __METHOD__ . PHP_EOL;
    }

    function h1() {
        echo __METHOD__ . PHP_EOL;
    }

    protected function j1() {
        echo __METHOD__ . PHP_EOL;
    }

    /*private */function k1() {
        echo __METHOD__ . PHP_EOL;
    }
}

$b = new B;
$b->f();
$b->g();
$b->h();
$b->j();
$b->k();
