<?php
namespace Hzw\DesignPatterns\Decorator;

abstract class Beverage {
    const TALL = 0;
    const GRANDE = 1;
    const VENTI = 2;

    private $sizeStr = [
        self::TALL => 'TALL',
        self::GRANDE => 'GRANDE',
        self::VENTI => 'VENTI',
    ];

    private $size;
    private $costs;
    private $description;

    public function __construct($size, $costs, $description = 'Unknown Beverage') {
        $this->size = $size;
        $this->costs = $costs;
        $this->description = $description;
    }

    public function getSize() {
        return $this->size;
    }

    public function getDescription() {
        $size = $this->sizeStr[$this->size];
        return "$this->description($size)";
    }

    public function cost() {
        return $this->costs[$this->size];
    }
}
