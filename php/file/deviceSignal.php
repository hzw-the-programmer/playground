<?php
if ($argc !== 2) {
    die("{$argv[0]} file\n");
}

$fn = $argv[1];

$f = fopen($fn, 'r');
while ($l = fgets($f)) {
    if (preg_match('/sn:(\d+).*signal:(-?\d+)/', $l, $matches) === 1) {
        echo "sn: {$matches[1]}, signal: {$matches[2]}\n";
    }
}
