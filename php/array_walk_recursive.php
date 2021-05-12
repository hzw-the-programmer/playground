<?php
$a = [1, 2, 3, 4, [5, 6, [7, 8, 9]]];
array_walk_recursive($a, function($item, $index) {
    echo "$index:$item\n";
});
