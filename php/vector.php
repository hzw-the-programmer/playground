<?php
function add($v1, $v2) {
    return array_map(function($it1, $it2) {
        return $it1 + $it2;
    }, $v1, $v2);
}

function scale_add($v1, $v2, $s) {
    return array_map(function($it1, $it2) use ($s) {
        return $it1 + $it2 * $s;
    }, $v1, $v2);
}

function sub($v1, $v2) {
    return array_map(function($it1, $it2) {
        return $it1 - $it2;
    }, $v1, $v2);
}

function len($v) {
    return sqrt(len_square($v));
}

function len_square($v) {
    return array_reduce($v, function($acc, $it) {
        return $acc + $it * $it;
    }, 0);
}

function mul($v1, $v2) {
    return array_map(function($it1, $it2) {
        return $it1 * $it2;
    }, $v1, $v2);
}

function div($v1, $v2) {
    return array_map(function($it1, $it2) {
        return $it1 / $it2;
    }, $v1, $v2);
}

function dot($v1, $v2) {
    $v = mul($v1, $v2);
    return array_reduce($v, function($acc, $it) {
        return $acc + $it;
    });
}

function scale($v, $s) {
    return array_map(function($it) use ($s) {
        return $it * $s;
    }, $v);
}

$v1 = [1, 2, 3];
$v2 = [4, 5, 6];

$r = [
    'add' => implode(', ', add($v1, $v2)),
    'scale_add' => implode(', ', scale_add($v1, $v2, 2)),
    'sub' =>  implode(', ', sub($v1, $v2, 2)),
    'len' => len($v1),
    'mul' => implode(', ', mul($v1, $v2)),
    'div' => implode(', ', div($v1, $v2)),
    'dot' => dot($v1, $v2),
    'scale' => implode(',', scale($v1, 2)),
];

$w = array_reduce(array_keys($r), function($acc, $it) {
    $len = strlen($it);
    return max($acc, $len);
}, 0);

foreach ($r as $key => $value) {
    echo sprintf("%{$w}s: %s\n", $key, $value);
}
