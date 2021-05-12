<?php
class Getway {
    private static $profile;

    private $id;
    private $freq;
    private $freqIndex;
    private $nodes;
    

    public function __construct($id, $freq) {
        $this->id = $id;
        $this->freq = $freq;
    }

    public function addNode($node) {
        $this->allocResource();
    }

    public function removeNode($node) {
        $this->freeResource();
    }

    public function allocResource() {
    }

    public function freeResource() {
    }
}

$gw = new Getway($id, $freq);

