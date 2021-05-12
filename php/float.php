<?php
$a = 1.23456789;
$b = 1.23456780;
$epsilon = 0.00001;

echo "$a\n";
echo "$b\n";
echo "$epsilon\n";
echo ($a - $b) . PHP_EOL;

echo ($a == $b ? 'true' : 'false') . PHP_EOL;
echo ($a === $b ? 'true' : 'false') . PHP_EOL;

if (abs($a - $b) < $epsilon) {
    echo "true\n";
}

$a = 2;
$b = 2.0;
echo ($a === $b ? 'true' : 'false') . PHP_EOL;
echo ($a == $b ? 'true' : 'false') . PHP_EOL;
echo 1 / 0.00000000000000000000000000001 . PHP_EOL;
echo 1 / ($a - $b) . PHP_EOL;
