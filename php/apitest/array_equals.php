<?php
$a = [1, 2, 3];
$b = [1, 2, 3];
var_dump($a == $b);
var_dump($a === $b);

$b = $a;
var_dump($a == $b);
var_dump($a === $b);
