<?php
if ($argc !== 3) {
    die("{$argv[0]} task file\n");
}

$task = $argv[1];
$file = $argv[2];

$f = fopen($file, 'r');

$sum = 0;
$count = 0;
while ($l = fgets($f)) {
    if (preg_match("/onTask\({$task}\): (.+)/", $l, $matches)) {
        $count++;
        $sum += $matches[1];
    }
}

if ($count !== 0) {
    $avg = $sum / $count;
} else {
    $avg = 0;
}

$w = max(strlen('sum'), strlen('count'), strlen('avg'));
echo sprintf("%{$w}s: %s\n", 'sum', $sum);
echo sprintf("%{$w}s: %s\n", 'count', $count);
echo sprintf("%{$w}s: %s\n", 'avg', $avg);
