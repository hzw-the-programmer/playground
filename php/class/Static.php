<?php
class Task {
    static function create($name) {
        return new static($name);
    }

    function __construct($name) {
        $this->name = $name;
    }
}

var_dump(Task::create('hzw'));
