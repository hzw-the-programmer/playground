<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

use Hzw\Channel;
use Hzw\Utils;
use Hzw\TaskManager;
use Hzw\Task\HBUpdateAvgData;
use Hzw\Task\HBUpdateStatusSignal;

class HeartBeatWithData extends Cmd {
    const CMD = Cmd::HeartBeatWithData;

    private $seq;
    private $channels;

    public function __construct($seq, $channels) {
        $this->seq = $seq;
        $this->channels = $channels;
    }

    public function getSeq() {
        return $this->seq;
    }

    public function getChannels() {
        return $this->channels;
    }

    public function getSignal() {
        return 1;
    }

    public function pack() {
        $bin = pack('C', self::CMD);
        $bin .= pack('V', $this->seq);
        $bin .= Utils::packHeartBeatWithData($this->channels);
        return $bin;
    }

    public static function unpack($bin) {
        $cmd = unpack('C', $bin)[1];
        if ($cmd !== self::CMD) return null;
        $bin = substr($bin, 1);
        $seq = unpack('V', $bin)[1];
        $bin = substr($bin, 4);
        return new self($seq, Utils::unpackHeartBeatWithData($bin));
    }

    public function process($cntx, $packet) {
        $header = $packet->getHeader();
        $pcdt = $packet->getPcdt();

        $sns = $header->getSns();
        $dt = $header->getDt();

        $sn = $sns[count($sns) - 1];

        $signal = $this->getSignal();
        if ($extraInfo = $packet->getExtraInfo()) {
            $signal = $extraInfo->rssi;
        }

        TaskManager::realtimeTask(new HBUpdateStatusSignal(
            $sn, $this->getSeq(), $dt, $pcdt, $this->getChannels(), $signal
        ));

        $devMgr = $cntx->getDeviceMgr();
        $device = $devMgr->getDevice($sn);
        $this->exchangeAvgData($device);
        $preHbSeq = $device->getHbSeq();
        $preHbDt = $device->getHbDt();
        if ($preHbSeq !== null) {
            $preRound = intval($preHbSeq / 10);
            $preCount = $preHbSeq % 10;
            $curRound = intval($this->seq / 10);
            $curCount = $this->seq % 10;
            if ($preRound !== $curRound) {
                $preHbDt += (9 - $preCount) * 60;
                TaskManager::realtimeTask(new HBUpdateAvgData(
                    $sn, $preHbSeq, $preHbDt, $pcdt, $this->getChannels()
                ));
            }
        }
        $device->setHbSeq($this->seq);
        $device->setHbDt($dt);
    }

    private function exchangeAvgData($device) {
        foreach ($this->channels as $channel) {
            $slot = $channel->getSlot();
            $port = $channel->getPort();
            $chan = $device->getChannel($slot, $port);
            if (!$chan) {
                $chan = new Channel($slot, $port, $channel->getType());
                $device->addChannel($chan);
            }
            $avgData = $chan->getAvgData();
            $chan->setAvgData($channel->getAvgData());
            $channel->setAvgData($avgData);
        }
    }
}
