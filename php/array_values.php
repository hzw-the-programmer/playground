<?php
$a = [1, 2, [3, 4], [5, 6, [7, 8]]];
$b = array_values($a);
$c = array_keys($a);
print_r($a);
print_r($b);
print_r($c);
echo ($a === $b ? 'true' : 'false') . "\n";
