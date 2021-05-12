<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
class Device {
    public $id;
    public $sn;
    public $ip;
    public $port;
    public $deveui;
    public $time;
    public $channels;

    public function __construct($sn) {
        $this->sn = $sn;
    }
}
