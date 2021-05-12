<?php
require __DIR__ . '/vendor/autoload.php';

use Hzw\DesignPatterns\Decorator\Beverage;
use Hzw\DesignPatterns\Decorator\DarkRoast;
use Hzw\DesignPatterns\Decorator\Espresso;
use Hzw\DesignPatterns\Decorator\HouseBlend;
use Hzw\DesignPatterns\Decorator\Mocha;
use Hzw\DesignPatterns\Decorator\Soy;
use Hzw\DesignPatterns\Decorator\Whip;

foreach ([DarkRoast::class, Espresso::class, HouseBlend::class] as $class) {
    foreach ([Beverage::TALL, Beverage::GRANDE, Beverage::VENTI] as $size) {
        $beverage = new $class($size);
        echo sprintf("%s $%0.2f\n", $beverage->getDescription(), $beverage->cost());
        foreach ([Mocha::class, Soy::class, Whip::class] as $decorator) {
            $beverage = new $decorator($beverage);
            echo sprintf("%s $%0.2f\n", $beverage->getDescription(), $beverage->cost());
        }
    }
}
