<?php
namespace Hzw\DesignPatterns\Decorator;

class HouseBlend extends Beverage {
    public function __construct($size) {
        $costs = [
            self::TALL => .79,
            self::GRANDE => .89,
            self::VENTI => .99,
        ];
        parent::__construct($size, $costs, 'House Blend Coffee');
    }
}
