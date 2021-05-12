<?php
$a = 1;
$func = function() use ($a) {
//$func = function() use (&$a) {
    echo "$a\n";
    $a = 3;
};
$a = 2;
$func();
echo "$a\n";

echo "\n";

$b = new stdClass();
$b->v = 1;
$func1 = function() use ($b) {
    echo "{$b->v}\n";
    $b->v = 3;
};
$b->v = 2;
$func1();
echo "{$b->v}\n";
