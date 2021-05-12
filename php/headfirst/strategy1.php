<?php
require __DIR__ . '/vendor/autoload.php';

use Hzw\DesignPatterns\Strategy\MallardDuck;
use Hzw\DesignPatterns\Strategy\FlyNoWay;
use Hzw\DesignPatterns\Strategy\Squeak;
use Hzw\DesignPatterns\Strategy\MuteQuack;

$duck = new MallardDuck();
$duck->display();
$duck->fly();
$duck->quack();
echo "\n";
$duck->setFlyBehavior(new FlyNoWay());
$duck->setQuackBehavior(new Squeak());
$duck->display();
$duck->fly();
$duck->quack();
echo "\n";
$duck->setQuackBehavior(new MuteQuack());
$duck->display();
$duck->fly();
$duck->quack();
echo "\n";