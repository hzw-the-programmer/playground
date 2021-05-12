<?php
if (1) {
    $a = [1, 2, 3];
    foreach ($a as &$v) {
        $v++;
    }
    unset($v);
    print_r($a);

    foreach ($a as $v) {
        print_r($a);
    }
} else {
    $a = [1, 2, 3];

    foreach ($a as &$v) {
        $v++;
    }
    unset($v);

    print_r($a);
    $v++;
    print_r($a);    
}
