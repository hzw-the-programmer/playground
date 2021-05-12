<?php
class Obj {
    private static $count = 0;
    function __construct() {
        $this->index = ++self::$count;
        echo $this->index . ' ' . __METHOD__ . PHP_EOL;
    }
    function __destruct() {
        echo $this->index . ' ' . __METHOD__ . PHP_EOL;
    }
    function __toString() {
        return $this->index . '';
    }
}

$a = [new Obj(), new Obj(), new Obj(), new Obj(), new Obj()];

/*
foreach ($a as $k => $v) {
    echo $v . PHP_EOL;
    if ($k === 0) {
        unset($a[1], $a[2], $a[3]);
        echo implode(',', $a) . PHP_EOL;
    }
}

echo implode(',', $a) . PHP_EOL;
*/

foreach ($a as $k => $v) {
    echo $v . PHP_EOL;
    var_dump($v === $a[$k]);
    $a[$k + 1] = new Obj();
    echo implode(',', $a) . PHP_EOL;
}
