<?php
class A {
    function __construct($v) {
        $this->v = $v;
    }

    function __destruct() {
        echo __METHOD__ . " $this->v\n";
    }

    function getClosure() {
        return function() {
            echo "{$this->v}\n";
        };
    }
}

$a = new A('a');
$b = new A('b');

$closure = $a->getClosure();

echo "a is set to null\n";
$a = null;

echo "************************\n";
$closure();
$closure->call($b);

echo "************************\n";
$c1 = $closure->bindTo($b);
$closure();
$c1();

$c2 = Closure::bind($c1, $b);

echo "set closure to null\n";
$closure = null;

echo "set b to null\n";
$b = null;

echo "set c1 to null\n";
$c1 = null;

$c2();
echo "set c2 to null\n";
$c2 = null;
echo "end\n";
