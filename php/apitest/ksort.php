<?php
$a = [0 => 'zero', 2 => 'two', 1 => 'one', 3 => 'three'];
ksort($a);
print_r($a);
krsort($a);
print_r($a);
