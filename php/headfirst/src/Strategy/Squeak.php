<?php
namespace Hzw\DesignPatterns\Strategy;

class Squeak implements QuackBehavior {
    public function quack() {
        echo "squeak\n";
    }
}
