<?php
namespace Hzw\DesignPatterns\Decorator;

class Soy extends Beverage {
    private static $costs = [
        self::TALL => .05,
        self::GRANDE => .15,
        self::VENTI => .25,
    ];

    private $beverage;

    public function __construct($beverage) {
        $this->beverage = $beverage;
    }

    public function getSize() {
        return $this->beverage->getSize();
    }

    public function getDescription() {
        return $this->beverage->getDescription() . ', Soy';
    }

    public function cost() {
        return $this->beverage->cost() + self::$costs[$this->getSize()];
    }
}
