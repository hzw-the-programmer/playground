<?php
$a = [1, 2, 3];
$b = ['one', 'two', 'three'];

$c = array_map(null, $b, $a);
print_r($c);

$c = array_map(function($a, $b) {
    return "$b: $a";
}, $a, $b);
print_r($c);

function cubic($e) {
    return $e * $e *$e;
}

$c = array_map('cubic', $a);
print_r($c);

$func = function($v) {
    return $v * 2;
};

$c = array_map($func, range(1, 5));
print_r($c);
