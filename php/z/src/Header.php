<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

class Header {
    private $sns;
    private $seq;
    private $dt;
    private $from;
    private $ver;
    private $len;

    function __construct($sns, $seq, $dt, $from = 'S', $ver = '4', $len = 0) {
        $this->sns = $sns;
        $this->seq = $seq;
        $this->dt = $dt;
        $this->from = $from;
        $this->ver = $ver;
        $this->len = $len;
    }

    function setSns($sns) {
        $this->sns = $sns;
    }

    function getSns() {
        return $this->sns;
    }

    function setSeq($seq) {
        $this->seq = $seq;
    }

    function getSeq() {
        return $this->seq;
    }

    function setDt($dt) {
        $this->dt = $dt;
    }

    function getDt() {
        return $this->dt;
    }

    function setFrom($from) {
        $this->from = $from;
    }

    function getFrom() {
        return $this->from;
    }

    function setVer($ver) {
        $this->ver = $ver;
    }

    function getVer() {
        return $this->ver;
    }

    function setLen($len) {
        $this->len = $len;
    }

    function getLen() {
        return $this->len;
    }

    function __toString() {
        $str = '{';
        $str .= Utils::snsStr($this->sns);
        $str .= ",{$this->seq}";
        $str .= ',' . date('Y-m-d H:i:s', $this->dt);
        $str .= '}';
        return $str;
    }
}
