<?php
class Observable {
    private $changed = FALSE;
    private $observers = [];

    public function addObserver($o) {
        if (in_array($o, $this->observers)) return;
        $this->observers[] = $o;
    }

    public function deleteObserver($o) {
        $key = array_search($o, $this->observers);
        if ($key === FALSE) return;
        unset($this->observers[$key]);
    }

    public function notifyObservers($obj) {
        for ($i = count($this->observers) - 1; $i >= 0; $i--) {
            $this->observers[$i]->update($this, $obj);
        }
    }
}
