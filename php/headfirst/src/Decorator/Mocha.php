<?php
namespace Hzw\DesignPatterns\Decorator;

class Mocha extends Beverage {
    private static $costs = [
        self::TALL => .10,
        self::GRANDE => .20,
        self::VENTI => .30,
    ];

    private $beverage;

    public function __construct($beverage) {
        $this->beverage = $beverage;
    }

    public function getSize() {
        return $this->beverage->getSize();
    }

    public function getDescription() {
        return $this->beverage->getDescription() . ', Mocha';
    }

    public function cost() {
        return $this->beverage->cost() + self::$costs[$this->getSize()];
    }
}
