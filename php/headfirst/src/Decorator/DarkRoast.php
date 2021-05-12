<?php
namespace Hzw\DesignPatterns\Decorator;

class DarkRoast extends Beverage {
    public function __construct($size) {
        $costs = [
            self::TALL => .89,
            self::GRANDE => .99,
            self::VENTI => 1.09,
        ];
        parent::__construct($size, $costs, 'Dark Roast Coffee');
    }
}
