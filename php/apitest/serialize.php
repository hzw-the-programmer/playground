<?php
class Device implements Serializable {
    //public $sn;
    //public $ip;
    //public $port;
    public $pcdt;

    public function __construct($sn) {
        $this->sn = $sn;
        echo "$this->sn constructed\n";
    }

    public function __destruct() {
        echo "$this->sn destructed\n";
    }

    public function serialize() {
        return serialize([$this->sn, $this->ip, $this->port]);
    }

    public function unserialize($serialized) {
        list($this->sn, $this->ip, $this->port) = unserialize($serialized);
    }
}

$device = new Device(20170123000001);
$device->ip = '192.168.1.100';
$device->port = 19268;
$device->pcdt = time();
$devices[$device->sn] = $device;

$device = new Device(20170123000002);
$device->ip = '192.168.1.101';
$device->port = 19268;
$devices[$device->sn] = $device;

//$device = null;

//echo "before serailize\n";
//file_put_contents('devices', serialize($devices));
//echo "after serailize\n";

//$devices = null;

echo "before unserialize\n";
if (is_readable('devices')) {
    $devices = unserialize(file_get_contents('devices'));
} else {
    $devices = [];
}

echo "after unserialize\n";

var_dump($devices);
