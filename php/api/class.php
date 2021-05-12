<?php
class A {
    public $pub = 'public';
    protected $pro = 'protected';
    private $pri = 'private';

    function print() {
        echo $this->pub . PHP_EOL;
        echo $this->pro . PHP_EOL;
        echo $this->pri . PHP_EOL;
    }

    function getPri() {
        return $this->pri;
    }

    function setPri($pri) {
        $this->pri = $pri;
    }

    function setPro($pro) {
        $this->pro = $pro;
    }
}

class B extends A {
    public $pub = 'b public';
    protected $pro = 'b protected';
    private $pri = 'b private';

    function printb() {
        echo $this->pub . PHP_EOL;
        echo $this->pro . PHP_EOL;
        echo $this->pri . PHP_EOL;
    }

    function setPrib($pri) {
        $this->pri = $pri;
    }

    function setProb($pro) {
        $this->pro = $pro;
    }
}

$a = new A;
echo $a->pub . PHP_EOL;
//echo $a->pro . PHP_EOL;
//echo $a->pri . PHP_EOL;
$a->print();

$b = new B;
echo $b->pub . PHP_EOL;
//echo $b->pro . PHP_EOL;
//echo $b->pri . PHP_EOL;
$b->print();
$b->printb();

echo "\n***private***\n";
$b->setPri('haha');
$b->print();
$b->printb();

$b->setPrib('hehe');
$b->print();
$b->printb();
echo "***private***\n";

echo "\n***protected***\n";
$b->setPro('aaaaaa');
$b->print();
$b->printb();

$b->setProb('bbbbb');
$b->print();
$b->printb();
echo "***protected***\n";
