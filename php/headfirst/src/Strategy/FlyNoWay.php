<?php
namespace Hzw\DesignPatterns\Strategy;

class FlyNoWay implements FlyBehavior {
    public function fly() {
        echo "I can't fly\n";
    }
}
