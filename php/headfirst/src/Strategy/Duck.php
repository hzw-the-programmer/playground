<?php
namespace Hzw\DesignPatterns\Strategy;

abstract class Duck {
    protected $flyBehavior;
    protected $quackBehavior;

    public abstract function display();

    public function setFlyBehavior($flyBehavior) {
        $this->flyBehavior = $flyBehavior;
    }

    public function setQuackBehavior($quackBehavior) {
        $this->quackBehavior = $quackBehavior;
    }

    public function fly() {
        $this->flyBehavior->fly();
    }

    public function quack() {
        $this->quackBehavior->quack();
    }

    public function swim() {
        echo "All ducks float, even decoys!\n";
    }
}
