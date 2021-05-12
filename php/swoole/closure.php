<?php
/*
class foo {
    public $bar;
}

$foo = new foo;
$foo->bar = "zhiwen he";

echo "before tick\n";
Swoole\Timer::tick(1000, function() use ($foo) {
    echo $foo->bar . "\n";
});
echo "after tick\n";

$foo->bar = "he zhiwen";
*/

/*
$foo = [1, 2];
echo "before tick\n";
Swoole\Timer::tick(1000, function() use ($foo) {
    var_dump($foo);
});
echo "after tick\n";
$foo[] = 3;
*/

/*
$foo = 1;
echo "before tick\n";
Swoole\Timer::tick(1000, function() use ($foo) {
    echo "{$foo}\n";
});
echo "after tick\n";
$foo = 2;
*/

/*
$foo = 1;
$bar = $foo;
$foo = 2;
echo "$bar\n";
*/

/*
$foo = [1, 2];
$bar = $foo;
$foo[] = 3;
print_r($foo);
print_r($bar);
*/

class foo {
    public $bar;
}

$foo = new foo;
$foo->bar = "zhiwen he";
$bar = $foo;
$foo->bar = "he zhiwen";

var_dump($foo);
var_dump($bar);
