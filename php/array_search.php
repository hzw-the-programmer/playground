<?php
$a = [1, 2, [3, 4], [5, 6, [7, 8]]];
$r = array_search([5, 6,[7, 8]], $a);
echo "$r\n";
