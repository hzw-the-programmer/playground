<?php
namespace Hzw\DesignPatterns\Decorator;

class Whip extends Beverage {
    private static $costs = [
        self::TALL => .00,
        self::GRANDE => .10,
        self::VENTI => .20,
    ];

    private $beverage;

    public function __construct($beverage) {
        $this->beverage = $beverage;
    }

    public function getSize() {
        return $this->beverage->getSize();
    }

    public function getDescription() {
        return $this->beverage->getDescription() . ', Whip';
    }

    public function cost() {
        return $this->beverage->cost() + self::$costs[$this->getSize()];
    }
}
