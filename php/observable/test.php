<?php
include "./Observable.php";
include "./Observer.php";

class Subject extends Observable {
    private $data;

    public function setData($data) {
        if ($this->data == $data) return;
        $this->data = $data;
        $this->notifyObservers(NULL);
    }

    public function getData() {
        return $this->data;
    }
}

class Observer1 implements Observer {
    public function update($observable, $obj) {
        echo "Observer1::update " . $observable->getData() . "\n";
    }
}

class Observer2 implements Observer {
    public function update($observable, $obj) {
        echo "Observer2::update " . $observable->getData() . "\n";
    }
}

$sub = new Subject();
$obs1 = new Observer1();
$sub->addObserver($obs1);
$obs2 = new Observer2();
$sub->addObserver($obs2);

$sub->setData("zhiwen he");
