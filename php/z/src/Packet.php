<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

use Hzw\Cmd\SetSn;
use Hzw\Cmd\GetSn;
use Hzw\Cmd\SetWorkTime;
use Hzw\Cmd\GetWorkTime;
use Hzw\Cmd\Calibration;
use Hzw\Cmd\DeviceStandbyHB;
use Hzw\Cmd\Ack;
use Hzw\Cmd\StatusData;
use Hzw\Cmd\Data;
use Hzw\Cmd\PowerOff;
use Hzw\Cmd\PowerOn;
use Hzw\Cmd\DeviceInfo;
use Hzw\Cmd\HeartBeatWithData;
use Hzw\Cmd\Open;
use Hzw\Cmd\Close;
use Hzw\Cmd\Status;
use Hzw\Cmd\GetParam;
use Hzw\Cmd\SetParam;
use Hzw\Cmd\HeartBeat;

class Packet {
    const OK = 0x00;
    const TOO_SHORT = 0x01;
    const WRONG_LEN = 0x02;
    const WRONG_CHKSUM = 0x03;
    const NO_BODY_UNPACKER = 0x04;

    private static $codes = [
        self::OK => 'OK',
        self::TOO_SHORT => 'TOO_SHORT',
        self::WRONG_LEN => 'WRONG_LEN',
        self::WRONG_CHKSUM => 'WRONG_CHKSUM',
        self::NO_BODY_UNPACKER => 'NO_BODY_UNPACKER'
    ];

    private $header;
    private $body;
    private static $bodyUnpackers = [];

    private $code;

    private $source = [];
    private $target = [];
    private $pcdt;
    private $extraInfo;

    private $unpackTime;

    function __construct($header, $body, $code = self::OK) {
        $this->header = $header;
        $this->body = $body;
        $this->code = $code;
    }

    function getHeader() {
        return $this->header;
    }

    function getBody() {
        return $this->body;
    }

    function isValid($from = '') {
        return $this->code == Packet::OK && (!$from || $this->header->getFrom() == $from);
    }

    public function setSource($source) {
        $this->source = $source;
        return $this;
    }

    public function getSource() {
        return $this->source;
    }

    public function setTarget($target) {
        $this->target = $target;
        return $this;
    }

    public function getTarget() {
        return $this->target;
    }

    public function setPcdt($pcdt) {
        $this->pcdt = $pcdt;
        return $this;
    }

    public function getPcdt() {
        return $this->pcdt;
    }

    public function setExtraInfo($extraInfo) {
        $this->extraInfo = $extraInfo;
        return $this;
    }

    public function getExtraInfo() {
        return $this->extraInfo;
    }

    function pack() {
        $sns = $this->header->getSns();
        $sn = array_shift($sns);

        $sn = Utils::packSn($sn);

        $seq = Utils::packSeq($this->header->getSeq());
        
        $dt = Utils::packDt($this->header->getDt());
        
        $sns = Utils::packDot($sns);
        
        $bin = $sn . $seq . $dt . $sns . $this->body->pack();
        $bin .= pack('C', Utils::chksum($bin));
        $bin = Utils::packLen(strlen($bin)) . $bin;
        $bin = pack('CC', ord($this->header->getFrom()), ord($this->header->getVer())) . $bin;
        
        return $bin;
    }

    static function unpack($bin)
    {
        $start = microtime(true);

        if (strlen($bin) < 4) {
            return new Packet(null, null, Packet::TOO_SHORT);
        }
        
        $from = chr(unpack('C', $bin)[1]);
        $ver = chr(unpack('C', $bin[1])[1]);
        $bin = substr($bin, 2);

        $len = Utils::unpackLen($bin);
        $bin = substr($bin, Utils::LEN_SIZE);
        if (strlen($bin) != $len) {
            return new Packet(null, null, Packet::WRONG_LEN);
        }

        $chksum = unpack('C', $bin[$len - 1])[1];
        $bin = substr($bin, 0, $len - 1);
        if (Utils::chksum($bin) != $chksum) {
            return new Packet(null, null, Packet::WRONG_CHKSUM);
        }

        $sn = Utils::unpackSn($bin);
        $bin = substr($bin, Utils::SN_SIZE);

        $seq = Utils::unpackSeq($bin);
        $bin = substr($bin, Utils::SEQ_SIZE);

        $dt = Utils::unpackDt($bin);
        $bin = substr($bin, Utils::DT_SIZE);

        $sns = Utils::unpackDot($bin);
        $bin = substr($bin, count($sns) * (Utils::SN_SIZE + 1));
        array_unshift($sns, $sn);

        $header = new Header($sns, $seq, $dt, $from, $ver, $len);
        
        $body = Packet::unpackBody($bin);
        if (!$body) {
            return new Packet(null, null, Packet::NO_BODY_UNPACKER);
        }

        $packet = new Packet($header, $body);

        $packet->unpackTime = microtime(true) - $start;
        
        return $packet;
    }

    static function unpackBody($bin) {
        foreach (Packet::$bodyUnpackers as $unpacker) {
            $body = $unpacker::unpack($bin);
            if ($body) return $body;
        }
        return null;
    }

    static function registerBodyUnpacker($unpacker) {
        $key = array_search($unpacker, Packet::$bodyUnpackers);
        if ($key !== false) return;
        Packet::$bodyUnpackers[] = $unpacker;
    }

    static function unregisterBodyUnpacker($unpacker) {
        $key = array_search($unpacker, Packet::$bodyUnpackers);
        if ($key === false) return;
        unset(Packet::$bodyUnpackers[$key]);
    }

    static function init() {
        Packet::registerBodyUnpacker(SetSn::class);
        Packet::registerBodyUnpacker(GetSn::class);
        Packet::registerBodyUnpacker(SetWorkTime::class);
        Packet::registerBodyUnpacker(GetWorkTime::class);
        Packet::registerBodyUnpacker(Calibration::class);
        Packet::registerBodyUnpacker(DeviceStandbyHB::class);
        Packet::registerBodyUnpacker(Ack::class);
        Packet::registerBodyUnpacker(StatusData::class);
        Packet::registerBodyUnpacker(Data::class);
        Packet::registerBodyUnpacker(PowerOff::class);
        Packet::registerBodyUnpacker(PowerOn::class);
        Packet::registerBodyUnpacker(DeviceInfo::class);
        Packet::registerBodyUnpacker(HeartBeatWithData::class);
        Packet::registerBodyUnpacker(Open::class);
        Packet::registerBodyUnpacker(Close::class);
        Packet::registerBodyUnpacker(Status::class);
        Packet::registerBodyUnpacker(GetParam::class);
        Packet::registerBodyUnpacker(SetParam::class);
        Packet::registerBodyUnpacker(HeartBeat::class);
    }

    static function deinit() {
        Packet::unregisterBodyUnpacker(SetSn::class);
        Packet::unregisterBodyUnpacker(GetSn::class);
        Packet::unregisterBodyUnpacker(SetWorkTime::class);
        Packet::unregisterBodyUnpacker(GetWorkTime::class);
        Packet::unregisterBodyUnpacker(Calibration::class);
        Packet::registerBodyUnpacker(DeviceStandbyHB::class);
        Packet::unregisterBodyUnpacker(Ack::class);
        Packet::unregisterBodyUnpacker(StatusData::class);
        Packet::unregisterBodyUnpacker(Data::class);
        Packet::unregisterBodyUnpacker(PowerOff::class);
        Packet::unregisterBodyUnpacker(PowerOn::class);
        Packet::unregisterBodyUnpacker(DeviceInfo::class);
        Packet::unregisterBodyUnpacker(HeartBeatWithData::class);
        Packet::unregisterBodyUnpacker(Open::class);
        Packet::unregisterBodyUnpacker(Close::class);
        Packet::unregisterBodyUnpacker(Status::class);
        Packet::unregisterBodyUnpacker(GetParam::class);
        Packet::unregisterBodyUnpacker(SetParam::class);
        Packet::unregisterBodyUnpacker(HeartBeat::class);
    }

    public function fromLora() {
        return isset($this->source['devaddr']);
    }

    public function isAck() {
        return $this->body::CMD === Ack::CMD
            && $this->body->getCmd() === Ack::CMD;
    }

    public function isStatus() {
        return $this->body::CMD === Status::CMD
            || $this->body::CMD === StatusData::CMD;
    }

    public function needAck() {
        if ($this->fromLora()) {
            //return $this->isStatus();
            return false;
        }
        return !$this->isAck();
    }

    public function packAck() {
        //$header = new Header($this->header->getSns(), 0, $this->pcdt);
        $header = new Header([$this->header->getSns()[0]], 0, $this->pcdt);
        $body = new Ack($this->header->getSeq(), 0x00, $this->body::CMD, null);
        $packet = new Packet($header, $body);
        return $packet->pack();
    }

    function __toString() {
        $str = '{';
        $str .= implode(':', $this->source);
        $str .= ',' . implode(':', $this->target);
        
        $str .= ',' . self::$codes[$this->code];
        if (!$this->isValid()) {
            goto end;
        }

        $str .= ",{$this->header}";
        $str .= ",{$this->body}";

        end:
        $str .= '}';
        return $str;
    }
}
