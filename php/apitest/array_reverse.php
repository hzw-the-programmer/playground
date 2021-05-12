<?php
$arr = ['a', 'b', 'c'];
print_r(array_reverse($arr));
print_r($arr);

$arr = ['a' => 'a', 'b' => 'b', 'c' => 'c'];
print_r(array_reverse($arr));
print_r($arr);

$arr = ['a' => 'a', 'b', 'c' => 'c', 'd'];
print_r(array_reverse($arr));
print_r(array_reverse($arr, true));
print_r($arr);
