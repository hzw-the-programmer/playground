<?php
/*
$a = [1, 2, 3];
$b = $a;
$b[0] = 0;
echo sprintf("%s\n", implode($a));
echo sprintf("%s\n", implode(',', $b));
*/

/*
$a = [1, 2, 3];
$b = &$a;
$b[0] = 0;
echo sprintf("%s\n", implode($a));
echo sprintf("%s\n", implode(',', $b));
*/

/*
$a = [1, 2, 3];
$fn = function() use ($a) {
    $a[0] = 100;
    echo sprintf("%s\n", implode(',', $a));
};
$a = [2, 3, 4];
$fn();
echo sprintf("%s\n", implode(',', $a));
*/

$a = [1, 2, 3];
$fn = function() use (&$a) {
    $a[0] = 100;
    echo sprintf("%s\n", implode(',', $a));
};
$a = [2, 3, 4];
$fn();
echo sprintf("%s\n", implode(',', $a));
