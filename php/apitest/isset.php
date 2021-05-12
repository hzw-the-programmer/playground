<?php
class A {
    public $a;
}

$a = new A;
var_dump($a);

var_dump(isset($a->a));
var_dump(isset($a->b));

unset($a->a);
var_dump($a);

$b = null;
var_dump(isset($b));
var_dump(isset($c));

$d;
var_dump($d);

$e = null;
var_dump($e);

$a->b = [];
var_dump(isset($a->b));
