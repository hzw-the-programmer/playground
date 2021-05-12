<?php
namespace Hzw\DesignPatterns\Strategy;

class MuteQuack implements QuackBehavior {
    public function quack() {
        echo "<<silence>>\n";
    }
}
