<?php
class People {
    private $name;

    function __construct($name) {
        $this->name = $name;
    }

    function __toString() {
        return "name: $this->name";
    }
}

class Student extends People {
    private $grade;

    function __construct($name, $grade) {
        parent::__construct($name);
        $this->grade = $grade;
    }

    function __toString() {
        return parent::__toString() . "\ngrade: $this->grade";
    }
}

$p = new People('hzw');
echo "$p\n";

echo "\n";

$s = new Student('hzw', '100');
echo "$s\n";
