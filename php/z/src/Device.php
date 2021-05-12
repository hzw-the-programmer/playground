<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

class Device {
    const OFFLINE = 256;
    const ONLINE = 259;
    const POWERON = 260;
    const POWEROFF = 261;

    static $statuses = [
        self::OFFLINE => 'OFFLINE',
        self::ONLINE => 'ONLINE',
        self::POWERON => 'POWERON',
        self::POWEROFF => 'POWEROFF'
    ];

    private $id;
    private $sn;
    private $psn = '';
    private $address = [];
    private $type;
    private $pcdt = 0;
    private $seq = 1;
    private $channels = [];

    private $params = [];
    private $periods = [];

    private $hbSeq;
    private $hbDt;

    function __construct($sn) {
        $this->sn = $sn;
    }

    static function createFromDb($row) {
        $device = new Device($row['sn']);
        $device->id = intval($row['did']);
        $device->psn = $row['psn'] ?: '';
        $ip = trim($row['ip']);
        if ($ip) {
            $device->address = ['ip' => $ip, 'port' => intval($row['port'])];
        } else if ($row['devaddr']) {
            $device->address = ['devaddr' => $row['devaddr']];
        }
        $device->type = trim($row['type']);
        return $device;
    }

    function clone($sn) {
        $device = clone $this;
        $device->sn = $sn;
        return $device;
    }

    function getId() {
        return $this->id;
    }

    function getSn() {
        return $this->sn;
    }

    function setPsn($psn) {
        $this->psn = $psn ?: '';
        return $this;
    }

    function getPsn() {
        return $this->psn;
    }

    function setAddress($address) {
        $this->address = $address;
        return $this;
    }

    function getAddress() {
        return $this->address;
    }

    function setPcdt($pcdt) {
        $this->pcdt = $pcdt;
        return $this;
    }

    function getPcdt() {
        return $this->pcdt;
    }

    function nextSeq() {
        if ($this->seq == PHP_INT_MAX) {
            $this->seq = 1;
        }
        return $this->seq++;
    }

    function addChannel($channel) {
        //if (!$this->getChannel($channel->getSlot(), $channel->getPort())) {
            $this->channels[] = $channel;
        //}
        return $this;
    }

    function removeChannel($channel) {
        list($index, $channel) = $this->getChannel($channel->getSlot(), $channel->getPort(), true);
        if ($channel) unset($this->channels[$index]);
        return $this;
    }

    function getChannel($slot, $port, $returnIndex = false) {
        foreach ($this->channels as $index => $channel) {
            if ($channel->getSlot() === $slot
                && $channel->getPort() === $port
            ) {
                return $returnIndex ? [$index, $channel] : $channel;
            }
        }
        return $returnIndex ? [null, null] : null;
    }

    function getChannels() {
        return $this->channels;
    }

    function getParam($type) {
        if (isset($this->params[$type])) {
            return $this->params[$type];
        }
        return null;
    }

    function setParam($type, $param) {
        if ($param) {
            $this->params[$type] = $param;
        } else {
            unset($this->params[$type]);
        }
        return $this;
    }

    function getPeriods() {
        return $this->periods;
    }

    function setPeriods($periods) {
        $this->periods = $periods;
        return $this;
    }

    function getHbSeq() {
        return $this->hbSeq;
    }

    function setHbSeq($hbSeq) {
        $this->hbSeq = $hbSeq;
        return $this;
    }

    function getHbDt() {
        return $this->hbDt;
    }

    function setHbDt($hbDt) {
        $this->hbDt = $hbDt;
        return $this;
    }
}
