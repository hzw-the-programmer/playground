<?php
namespace Hzw\DesignPatterns\Observer;

interface Subject {
    public function registerObserver($observer);
    public function removeObserver($observer);
    public function notifyObservers();
}
