<?php
$a = [1, 2, 3, 4, 5, [6, 7, 8]];
echo (in_array([6, 7, 8], $a) ? 'true' : 'false') . "\n";
echo (in_array([6, 7, 8, 9], $a) ? 'true' : 'false') . "\n";
echo (in_array([6, 7, 9], $a) ? 'true' : 'false') . "\n";
