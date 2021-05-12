<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

use Hzw\Utils;

class Ack extends Cmd {
    const CMD = Cmd::Ack;

    private $seq, $status, $cmd, $data;

    function __construct($seq, $status, $cmd, $data) {
        $this->seq = $seq;
        $this->status = $status;
        $this->cmd = $cmd;
        $this->data = $data;
    }

    function getSeq() {
        return $this->seq;
    }

    function getStatus() {
        return $this->status;
    }

    function getCmd() {
        return $this->cmd;
    }

    function getData() {
        return $this->data;
    }

    function pack() {
        $bin = pack('C', self::CMD);
        $bin .= Utils::packSeq($this->seq);
        $bin .= pack('C', $this->status);
        $bin .= pack('C', $this->cmd);

        switch ($this->cmd) {
            case GetSn::CMD:
                $bin .= Utils::packSns($this->data);
                break;

            case GetParam::CMD:
                $bin .= Utils::packParam($this->data);
                break;
            
            case GetWorkTime::CMD:
                $bin .= Utils::packPeriods($this->data);
                break;

            default:
                break;
        }
        return $bin;
    }

    static function unpack($bin) {
        $cmd = unpack('C', $bin)[1];
        if ($cmd != self::CMD) return null;
        $bin = substr($bin, 1);

        $seq = Utils::unpackSeq($bin);
        $bin = substr($bin, Utils::SEQ_SIZE);

        $status = unpack('C', $bin)[1];
        $bin = substr($bin, 1);

        $cmd = unpack('C', $bin)[1];
        $bin = substr($bin, 1);

        switch ($cmd) {
            case GetSn::CMD:
                $data = Utils::unpackSns($bin);
                break;

            case GetParam::CMD:
                $data = Utils::unpackParam($bin);
                break;

            case GetWorkTime::CMD:
                $data = Utils::unpackPeriods($bin);
                break;

            default:
                $data = null;
                break;
        }

        return new self($seq, $status, $cmd, $data);
    }

    function __toString() {
        $str = '{';
        $str .= "Ack";
        $str .= ",{$this->seq}";
        $str .= ",{$this->status}";
        $str .= sprintf(',0x%02x', $this->cmd);
        switch ($this->cmd) {
            case GetSn::CMD:
                $str .= '-' . Utils::snsStr($this->data);
                break;
        }
        $str .= '}';
        return $str;
    }
}
