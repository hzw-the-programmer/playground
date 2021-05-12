<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

class Channel {
    const GND_H = 0x08;
    const GND_L = 0x0C;
    const WS = 0x09;
    const VB = 0x0A;
    const TEMP = 0x0B;
    const HUMI = 0x0D;
    const ESI_V = 0x10;
    const ESI_R = 0x11;
    const ESD_V = 0x12;
    const ESD_R = 0x13;
    const INVALID = 0x00;

    static $types = [
        self::GND_H => 'GND_H',
        self::GND_L => 'GND_L',
        self::WS => 'WS',
        self::VB => 'VB',
        self::TEMP => 'TEMP',
        self::HUMI => 'HUMI',
        self::ESI_V => 'ESI_V',
        self::ESI_R => 'ESI_R',
        self::ESD_V => 'ESD_V',
        self::ESD_R => 'ESD_R',
        self::INVALID => 'INVALID',
    ];

    const OFF = 257;
    const ON = 258;

    private $id;
    private $slot;
    private $port;
    private $type;

    private $mid;
    private $pid;
    private $name;

    private $status;
    private $dt;
    private $seq;

    private $preStatus;
    private $preDt;
    private $preSeq;

    private $data;
    private $avgData;

    private $calibResult;
    private $calibData;

    function __construct($slot, $port, $type = Channel::INVALID) {
        $this->slot = $slot;
        $this->port = $port;
        $this->type = $type;
    }

    static function createFromDb($row) {
        $channel = new Channel(intval($row['slot']), intval($row['cport']), intval($row['ctype']));
        $channel->id = intval($row['cid']);

        $channel->mid = intval($row['mid']);
        $channel->pid = intval($row['pid']);
        $channel->name = $row['name'];

        $channel->status = intval($row['status']);
        $channel->dt = intval($row['dt']);
        $channel->seq = intval($row['seq']);
        $channel->preStatus = intval($row['preStatus']);
        $channel->preDt = intval($row['preDt']);
        $channel->preSeq = intval($row['preSeq']);

        return $channel;
    }

    function getId() {
        return $this->id;
    }

    function getSlot() {
        return $this->slot;
    }

    function getPort() {
        return $this->port;
    }

    function getType() {
        return $this->type;
    }

    function setType($type) {
        $this->type = $type;
        return $this;
    }

    function getMid() {
        return $this->mid;
    }

    function setMid($mid) {
        $this->mid = $mid;
        return $this;
    }

    function getPid() {
        return $this->pid;
    }

    function setPid($pid) {
        $this->pid = $pid;
        return $this;
    }

    function getName() {
        return $this->name;
    }

    function setName($name) {
        $this->name = $name;
        return $this;
    }

    function getStatus() {
        return $this->status;
    }

    function setStatus($status) {
        $this->status = $status;
        return $this;
    }

    public function getRealStatus() {
        $status = $this->status;

        if ($status > 255)
            return $status;

        switch ($this->type) {
            case Channel::WS:
                if ($status & 0x80) return 0x80;
                if (($status & 0x50) === 0x50) return 0x50;
                if ($status & 0x40) return 0x40;
                return $status & 0x20;
            default:
                return $status & 0x20;
        }
    }

    public function getLevel() {
        $status = $this->status;

        if ($status > 255)
            return 0;

        return $status & 0x07;
    }

    function getDt() {
        return $this->dt;
    }

    function setDt($dt) {
        $this->dt = $dt;
        return $this;
    }

    function getSeq() {
        return $this->seq;
    }

    function setSeq($seq) {
        $this->seq = $seq;
        return $this;
    }

    function getPreStatus() {
        return $this->preStatus;
    }

    function setPreStatus($preStatus) {
        $this->preStatus = $preStatus;
        return $this;
    }

    function getPreDt() {
        return $this->preDt;
    }

    function setPreDt($preDt) {
        $this->preDt = $preDt;
        return $this;
    }

    function getPreSeq() {
        return $this->preSeq;
    }

    function setPreSeq($preSeq) {
        $this->preSeq = $preSeq;
        return $this;
    }

    function getData() {
        return $this->data;
    }

    function setData($data) {
        $this->data = $data;
        return $this;
    }

    function getAvgData() {
        return $this->avgData;
    }

    function setAvgData($avgData) {
        $this->avgData = $avgData;
        return $this;
    }

    function getCalibResult() {
        return $this->calibResult;
    }

    function setCalibResult($calibResult) {
        $this->calibResult = $calibResult;
        return $this;
    }

    function getCalibData() {
        return $this->calibData;
    }

    function setCalibData($calibData) {
        $this->calibData = $calibData;
        return $this;
    }

    function __toString() {
        $str = '{';
        foreach ($this as $key => $value) {
            if (($key === 'status' || $key === 'data') && !is_numeric($value)) {
                continue;
            }

            if ($key === 'type') {
                $value = static::$types[$value];
            }

            $str .= "$key:$value,";
        }
        $str = rtrim($str, ',');
        $str .= '}';

        return $str;
    }
}
