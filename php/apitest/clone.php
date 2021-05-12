<?php
class SubObj {
    private static $count = 0;

    function __construct() {
        echo __METHOD__ . PHP_EOL;
        $this->index = ++self::$count;
    }

    function __clone() {
        echo __METHOD__ . PHP_EOL;
        $this->index = ++self::$count;
    }
}

class MainObj {
    public $subObj1;
    public $subObj2;

    function __construct() {
        echo __METHOD__ . PHP_EOL;
        $this->subObj1 = new SubObj();
        $this->subObj2 = new SubObj();
    }

    function __clone() {
        echo __METHOD__ . PHP_EOL;
        $this->subObj1 = clone $this->subObj1;
    }
}

$mainObj = new MainObj();
$cloneObj = clone $mainObj;
var_dump($mainObj === $cloneObj);
var_dump($mainObj->subObj1 === $cloneObj->subObj1);
var_dump($mainObj->subObj2 === $cloneObj->subObj2);
var_dump($mainObj, $cloneObj);
