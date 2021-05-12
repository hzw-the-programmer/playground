<?php
$a = range(1, 10);

function odd($v) {
    return $v & 1;
}

$even = function($v) {
    return !odd($v);
};

$b = array_filter($a, 'odd');
print_r($b);

$b = array_filter($a, $even);
print_r($b);

print_r($a);

$a = [0, 0.0, '0', '', [], null, false, '0.0'];
$b = array_filter($a);
print_r($b);