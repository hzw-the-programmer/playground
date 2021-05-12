<?php
namespace Hzw\DesignPatterns\Strategy;

class Quack implements QuackBehavior {
    public function quack() {
        echo "quack\n";
    }
}
