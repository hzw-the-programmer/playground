<?php
namespace Hzw\DesignPatterns\Strategy;

class MallardDuck extends Duck {
    public function __construct() {
        $this->flyBehavior = new FlyWithWings();
        $this->quackBehavior = new Quack();
    }

    public function display() {
        echo "I'm a real Mallard duck\n";
    }
}
