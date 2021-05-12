<?php
namespace Hzw\DesignPatterns\Decorator;

class Espresso extends Beverage {
    public function __construct($size) {
        $costs = [
            self::TALL => 1.89,
            self::GRANDE => 1.99,
            self::VENTI => 2.09,
        ];
        parent::__construct($size, $costs, 'Espresso');
    }
}
