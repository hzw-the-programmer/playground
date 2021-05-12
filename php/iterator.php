<?php
class MyIterator implements Iterator {
    private $pos = 0;
    private $arr = ['Warren Buffett', 'Charlie Munger'];

    function rewind() {
        var_dump(__METHOD__);
        $this->pos = 0;
    }

    function next() {
        var_dump(__METHOD__);
        $this->pos++;
    }

    function valid() {
        var_dump(__METHOD__);
        return isset($this->arr[$this->pos]);
    }

    function current() {
        var_dump(__METHOD__);
        return $this->arr[$this->pos];
    }

    function key() {
        var_dump(__METHOD__);
        return $this->pos;
    }
}

$it = new MyIterator();
foreach ($it as $key => $value) {
    echo "$key:$value\n";
}

echo "************************\n";

foreach ($it as $value) {
    echo "$value\n";
}
