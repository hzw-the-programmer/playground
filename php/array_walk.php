<?php
$a = [1, 2, 3, [4, 5]];
array_walk($a, function($item, $index) {
    if (is_array($item)) {
        echo "$index:";
        print_r($item);
        $item = 'hzw';
    } else {
        echo "$index:$item\n";
    }
});

print_r($a);

array_walk($a, function(&$item, $index) {
    $item = 'hzw';
});

print_r($a);