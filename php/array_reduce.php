<?php
$a = [1, 2, 3, 4, 5];
$b = array_reduce($a, function($carry, $item) {
    return $carry . $item;
});
echo "$b\n";

$b = array_reduce($a, function($carry, $item) {
    return $carry . $item;
}, 5);
echo "$b\n";
