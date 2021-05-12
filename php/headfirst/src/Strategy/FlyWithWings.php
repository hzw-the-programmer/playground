<?php
namespace Hzw\DesignPatterns\Strategy;

class FlyWithWings implements FlyBehavior {
    public function fly() {
        echo "I'm flying!!\n";
    }
}
