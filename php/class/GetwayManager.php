<?php
class GetwayManager {
    private $getways;

    public function init() {
        swoole_timer(100, [$this, 'send']);
    }

    public function processPacket($packet) {
        if (!isset($this->getways[$packet->getwayId])) {
            $this->getways[$packet->getwayId] = new Getway();
        }
        $getway = $this->getways[$packet->getwayId];
        // before getway process packet
        // ...
        $getway->processPacket($packet);
        // after getway process pacekt
        // ...
    }

    function send() {
        foreach ($this->getways as $getway) {
            if ($getway->wanttosend) {
                $getway->send();
            }
        }
    }
}