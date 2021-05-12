<?php
$a = 1;
$b = 2;

function test() {
    echo $a;
}

test();

function sum1() {
    global $a, $b;
    $b = $a + $b;
}

function sum2() {
    $GLOBALS['b'] = $GLOBALS['a'] + $GLOBALS['b'];
}

sum1();
echo "$b\n";
sum2();
echo "$b\n";

echo "******************\n";

function func1() {
    static $a = 0;
    echo "$a\n";
    $a++;
}

func1();
func1();
func1();

echo "******************\n";

function count_down() {
    static $count = 10;
    echo "$count\n";
    $count--;
    if ($count < 0) {
        return;
    }
    count_down();
}

count_down();

echo "******************\n";
