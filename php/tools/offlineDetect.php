<?php
if ($argc != 2) {
    die("$argv[0] file\n");
}

$f = fopen($argv[1], 'r');
while ($l = fgets($f)) {
    if (preg_match('/(.+), (.+), (.+), (.+)/', $l, $matches)) {
        $sn = $matches[1];
        $lastdt = $matches[2];
        $timeout = $matches[3];
        $chkdt = $matches[4];
    } else if (preg_match('/timeout: (.+)/', $l, $matches)) {
        $timeoutdt = $matches[1];
        if ($chkdt != $timeoutdt) {
            echo "$sn, $lastdt, $timeout, $chkdt, $timeoutdt\n";
        }
    }
}
