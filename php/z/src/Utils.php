<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

class Utils {
    const DOT = 0x2E;

    const SN_SIZE = 8;

    public static function packSn($sn) {
        return pack('J', $sn);
    }

    public static function unpackSn($bin) {
        $sn = unpack('J', $bin)[1];
        if ($sn === -1) $sn = 0;
        return sprintf('%014u', $sn);
    }

    public static function packSns($sns) {
        $bin = pack('C', count($sns));
        foreach ($sns as $sn) {
            $bin .= Utils::packSn($sn);
        }
        return $bin;
    }

    public static function unpackSns($bin) {
        $sns = [];
        $count = unpack('C', $bin)[1];
        $bin = substr($bin, 1);
        for ($i = 0; $i < $count; $i++) {
            $sns[] = Utils::unpackSn($bin);
            $bin = substr($bin, Utils::SN_SIZE);
        }
        return $sns;
    }

    public static function packDot($sns) {
        $bin = '';
        foreach ($sns as $sn) {
            $bin .= pack('C', Utils::DOT) . Utils::packSn($sn);
        }
        return $bin;
    }

    public static function unpackDot($bin) {
        $sns = [];
        while (strlen($bin) != 0 && unpack('C', $bin)[1] == Utils::DOT) {
            $bin = substr($bin, 1);
            $sns[] = Utils::unpackSn($bin);
            $bin = substr($bin, Utils::SN_SIZE);
        }
        return $sns;
    }

    const SEQ_SIZE = 4;

    public static function packSeq($seq) {
        return pack('N', $seq);
    }

    public static function unpackSeq($pkt) {
        return unpack('N', $pkt)[1];
    }

    const DT_SIZE = 6;

    public static function packDt($dt) {
        $lt = localtime($dt, true);
        $bin = pack('C', $lt['tm_year'] + 1900 - 2000);
        $bin .= pack('C', $lt['tm_mon'] + 1);
        $bin .= pack('C', $lt['tm_mday']);
        $bin .= pack('C', $lt['tm_hour']);
        $bin .= pack('C', $lt['tm_min']);
        $bin .= pack('C', $lt['tm_sec']);
        return $bin;
    }

    public static function unpackDt($bin) {
        $year = unpack('C', $bin)[1];
        $mon = unpack('C', $bin[1])[1];
        $day = unpack('C', $bin[2])[1];
        $hour = unpack('C', $bin[3])[1];
        $min = unpack('C', $bin[4])[1];
        $sec = unpack('C', $bin[5])[1];

        return mktime($hour, $min, $sec, $mon, $day, $year + 2000);
    }

    public static function chksum($bin) {
        $chksum = 0;
        for ($i = 0; $i < strlen($bin); $i++) {
            $chksum ^= ord($bin[$i]);
        }
        return $chksum;
    }

    const LEN_SIZE = 2;

    public static function packLen($len) {
        return pack('n', $len);
    }

    public static function unpackLen($bin) {
        return unpack('n', $bin)[1];
    }

    public static function packChannel($channel) {
        return pack('C', ($channel->getSlot() << 4) | $channel->getPort());
    }

    public static function unpackChannel($bin) {
        $sp = unpack('C', $bin)[1];
        $channel = new Channel($sp >> 4 & 0x0F, $sp & 0x0F);
        return $channel;
    }

    public static function packChannels($channels) {
        $bin = pack('C', count($channels));
        foreach ($channels as $channel) {
            $bin .= Utils::packChannel($channel);
        }
        return $bin;
    }

    public static function unpackChannels($bin) {
        $channels = [];
        $count = unpack('C', $bin)[1];
        $bin = substr($bin, 1);
        for ($i = 0; $i < $count; $i++) {
            $channel = Utils::unpackChannel($bin);
            $bin = substr($bin, 1);
            $channels[] = $channel;
        }
        return $channels;
    }

    public static function packStatus($channels) {
        $bin = pack('C', count($channels));

        foreach ($channels as $channel) {
            $bin .= Utils::packChannel($channel)
                . pack('C', $channel->getType())
                . pack('C', $channel->getStatus());
        }

        return $bin;
    }

    public static function unpackStatus($bin) {
        $channels = [];

        $count = unpack('C', $bin)[1];
        $bin = substr($bin, 1);

        for ($i = 0; $i < $count; $i++) {
            $channel = Utils::unpackChannel($bin);
            $channel->setType(unpack('C', $bin[1])[1])
                    ->setStatus(unpack('C', $bin[2])[1]);
            $bin = substr($bin, 3);

            $channels[] = $channel;
        }

        return $channels;
    }

    public static function packStatusData($channels) {
        $bin = pack('C', count($channels));

        foreach ($channels as $channel) {
            $data = Utils::convertData($channel->getType(), $channel->getData());

            $bin .= Utils::packChannel($channel)
                . pack('C', $channel->getType())
                . pack('C', $channel->getStatus())
                . pack('g'/*G*/, $data);
        }

        return $bin;
    }

    public static function unpackStatusData($bin) {
        $channels = [];

        $count = unpack('C', $bin)[1];
        $bin = substr($bin, 1);

        for ($i = 0; $i < $count; $i++) {
            $channel = Utils::unpackChannel($bin);
            $channel->setType(unpack('C', $bin[1])[1])
                    ->setStatus(unpack('C', $bin[2])[1]);
            $bin = substr($bin, 3);

            $data = unpack('g'/*'G'*/, $bin)[1];
            $bin = substr($bin, 4);
            $data = Utils::convertData($channel->getType(), $data, true);
            $channel->setData($data);

            $channels[] = $channel;
        }

        return $channels;
    }

    public static function packData($channels) {
        $bin = pack('C', count($channels));

        foreach ($channels as $channel) {
            $data = Utils::convertData($channel->getType(), $channel->getData());

            $bin .= pack('C', $channel->getSlot())
                . pack('C', $channel->getPort())
                . pack('C', $channel->getType())
                . pack('g'/*'G'*/, $data);
        }

        return $bin;
    }

    public static function unpackData($bin) {
        $channels = [];

        $count = unpack('C', $bin)[1];
        $bin = substr($bin, 1);

        for ($i = 0; $i < $count; $i++) {
            $slot = unpack('C', $bin)[1];
            $port = unpack('C', $bin[1])[1];
            $type = unpack('C', $bin[2])[1];
            $bin = substr($bin, 3);

            $data = unpack('g'/*'G'*/, $bin)[1];
            $bin = substr($bin, 4);
            $data = Utils::convertData($type, $data, true);

            $channel = new Channel($slot, $port, $type);
            $channel->setData($data);

            $channels[] = $channel;
        }

        return $channels;
    }

    public static function convertData($type, $data, $reverse = false) {
        switch ($type) {
            case Channel::GND_H:
                //$max = 875;
                $max = 900;
                if ($reverse) {
                    if ($data >= $max) {
                        $data = -999;
                    } else {
                        //$data = (($data * 10) / ($max - $data)) * 1000000;
                        $data = (($data * 50) / ($max - $data)) * 1000000;
                    }
                } else {
                    //$data = ($data * $max) / ($data + 10 * 1000000);
                    $data = ($data * $max) / ($data + 50 * 1000000);
                }
                break;

            case Channel::GND_L:
                $max = 1250;
                if ($reverse) {
                    if ($data >= $max) {
                        $data = -999;
                    } else {
                        $data = ($data * 200) / ($max - $data);
                    }
                } else {
                    $data = ($data * $max) / ($data + 200);
                }
                break;

            case Channel::WS:
                $max = 2450;
                //$max = 2500;
                if ($reverse) {
                    if ($data >= $max) {
                        $data = -999;
                    } else {
                        $data = (($data * 10) / ($max - $data)) * 1000000;
                        //$data = (($data * 12 - 5) / ($max - $data)) * 1000000;
                    }
                } else {
                    $data = ($data * $max) / ($data + 10 * 1000000);
                    //$data = ($data * $max + 5 * 1000000) / ($data + 12 * 1000000);
                }
                break;

            default:
                break;
        }

        return $data;
    }

    public static function packParam($param) {
        $bin = pack('C', $param->type);
        $bin .= pack('v'/*'n'*/, $param->analogInterval);

        $lowerLimit = $param->lowerLimit;
        $lowerLimit = Utils::convertLimit($param->type, $lowerLimit);
        $bin .= pack('g'/*'v''n'*/, $lowerLimit);

        $upperLimit = $param->upperLimit;
        $upperLimit = Utils::convertLimit($param->type, $upperLimit);
        $bin .= pack('g'/*'v''n'*/, $upperLimit);

        $bin .= pack('v'/*'n'*/, $param->alarmDelay);
        $bin .= pack('v'/*'n'*/, $param->level1Delay);
        $bin .= pack('v'/*'n'*/, $param->level2Delay);
        $bin .= pack('v'/*'n'*/, $param->level3Delay);

        return $bin;
    }

    public static function unpackParam($bin) {
        $type = unpack('C', $bin)[1];
        $bin = substr($bin, 1);

        $analogInterval = unpack('v'/*'n'*/, $bin)[1];
        $bin = substr($bin, 2);

        $lowerLimit = unpack('g'/*'v''n'*/, $bin)[1];
        $lowerLimit = Utils::convertLimit($type, $lowerLimit, true);
        $bin = substr($bin, 4/*2*/);

        $upperLimit = unpack('g'/*'v''n'*/, $bin)[1];
        $upperLimit = Utils::convertLimit($type, $upperLimit, true);
        $bin = substr($bin, 4/*2*/);

        $alarmDelay = unpack('v'/*'n'*/, $bin)[1];
        $bin = substr($bin, 2);

        $level1Delay = unpack('v'/*'n'*/, $bin)[1];
        $bin = substr($bin, 2);

        $level2Delay = unpack('v'/*'n'*/, $bin)[1];
        $bin = substr($bin, 2);

        $level3Delay = unpack('v'/*'n'*/, $bin)[1];

        return new Param($type, $analogInterval, $lowerLimit, $upperLimit,
            $alarmDelay, $level1Delay, $level2Delay, $level3Delay);
    }

    public static function convertLimit($type, $limit, $reverse = false) {
        switch ($type) {
            case Channel::GND_H:
                if ($reverse) {
                    //$limit = ($limit * 10) / (0.875 * 1000 - $limit);
                    $limit = ($limit * 50) / (0.9 * 1000 - $limit);
                } else {
                    //$limit = ((0.875 * $limit) / ($limit + 10)) * 1000;
                    $limit = ((0.9 * $limit) / ($limit + 50)) * 1000;
                }
                break;

            case Channel::GND_L:
                if ($reverse) {
                    $limit = ($limit * 200) / (1.25 * 1000 - $limit);
                } else {
                    $limit = ((1.25 * $limit) / ($limit + 200)) * 1000;
                }
                break;

            case Channel::WS:
                if ($reverse) {
                    //$limit = ($limit * 10) / (2.45 * 1000 - $limit);
                    $limit = ($limit * 12 - 2.5 * 1000 * 2) / (2.5 * 1000 - $limit);
                } else {
                    $limit = ((2.45 * $limit) / ($limit + 10)) * 1000;
                    //$limit = ((2.5 * ($limit + 2)) / ($limit + 12)) * 1000;
                }
                break;

            default:
                break;
        }

        //if ($reverse) return $limit;
        //return round($limit);
        return $limit;
    }

    public static function snsStr($sns) {
        $str = '[';
        foreach ($sns as $sn) {
            $str .= "'{$sn}',";
        }
        $str = trim($str, ',');
        $str .= ']';
        return $str;
    }

    public static function packPeriods($periods) {
        $bin = pack('C', count($periods));

        foreach ($periods as $period) {
            foreach ($period as $ts) {
                $lt = localtime($ts, true);
                $bin .= pack('C', $lt['tm_hour']);
                $bin .= pack('C', $lt['tm_min']);
            }
        }

        return $bin;
    }

    public static function unpackPeriods($bin) {
        $periods = [];

        $count = unpack('C', $bin)[1];
        $bin = substr($bin, 1);

        for ($i = 0; $i < $count; $i++) {
            $period = [];
            for ($j = 0; $j < 2; $j++) {
                $hour = unpack('C', $bin)[1];
                $min = unpack('C', $bin[1])[1];
                $bin = substr($bin, 2);
                $period[] = mktime($hour, $min);
            }
            $periods[] = $period;
        }

        return $periods;
    }

    public static function convertToChannels($channels) {
        $chans = [];
        foreach ($channels as $channel) {
            $chan = new Channel($channel->slot, $channel->port);
            if (isset($channel->type)) {
                $chan->setType($channel->type);
            }
            if (isset($channel->pid)) {
                $chan->setPid($channel->pid);
            }
            if (isset($channel->name)) {
                $chan->setName($channel->name);
            }
            $chans[] = $chan;
        }
        return $chans;
    }

    public static function packVersion($version) {
        list($major, $minor, $patch) = explode('.', $version);
        return pack('C', $major)
            . pack('C', $minor)
            . pack('C', $patch);
    }

    public static function unpackVersion($bin) {
        return unpack('C', $bin)[1] . '.'
            . unpack('C', $bin[1])[1] . '.'
            . unpack('C', $bin[2])[1];
    }

    public static function packHeartBeatWithData($channels) {
        $bin = pack('C', count($channels));

        foreach ($channels as $channel) {
            $type = $channel->getType();

            $data = $channel->getData();
            $data = Utils::convertData($type, $data);
            
            $avgData = $channel->getAvgData();
            $avgData = Utils::convertData($type, $avgData);

            $bin .= pack('C', $channel->getSlot())
                . pack('C', $channel->getPort())
                . pack('C', $type)
                . pack('C', $channel->getStatus())
                . pack('g', $data)
                . pack('g', $avgData);
        }

        return $bin;
    }

    public static function unpackHeartBeatWithData($bin) {
        $channels = [];

        $count = unpack('C', $bin)[1];
        $bin = substr($bin, 1);

        for ($i = 0; $i < $count; $i++) {
            $slot = unpack('C', $bin)[1];
            $port = unpack('C', $bin[1])[1];
            $type = unpack('C', $bin[2])[1];
            $bin = substr($bin, 3);

            $status = unpack('C', $bin)[1];
            $bin = substr($bin, 1);

            $data = unpack('g', $bin)[1];
            $data = Utils::convertData($type, $data, true);
            $bin = substr($bin, 4);

            $avgData = unpack('g', $bin)[1];
            $avgData = Utils::convertData($type, $avgData, true);
            $bin = substr($bin, 4);

            $channel = new Channel($slot, $port, $type);
            $channel->setStatus($status);
            $channel->setData($data);
            $channel->setAvgData($avgData);

            $channels[] = $channel;
        }

        return $channels;
    }

    public static function packCalibration($channels) {
        $bin = pack('C', count($channels));

        foreach ($channels as $channel) {
            $type = $channel->getType();

            $calibResult = $channel->getCalibResult();

            $calibData = $channel->getCalibData();
            //$calibData = Utils::convertData($type, $calibData);

            $bin .= pack('C', $channel->getSlot())
                . pack('C', $channel->getPort())
                . pack('C', $type)
                . pack('C', $calibResult)
                . pack('g', $calibData);
        }

        return $bin;
    }

    public static function unpackCalibration($bin) {
        $channels = [];

        $count = unpack('C', $bin)[1];
        $bin = substr($bin, 1);

        for ($i = 0; $i < $count; $i++) {
            $slot = unpack('C', $bin)[1];
            $port = unpack('C', $bin[1])[1];
            $type = unpack('C', $bin[2])[1];
            $bin = substr($bin, 3);

            $calibResult = unpack('C', $bin)[1];
            $bin = substr($bin, 1);

            $calibData = unpack('g', $bin)[1];
            //$calibData = Utils::convertData($type, $calibData, true);
            $bin = substr($bin, 4);

            $channel = new Channel($slot, $port, $type);
            $channel->setCalibResult($calibResult);
            $channel->setCalibData($calibData);

            $channels[] = $channel;
        }

        return $channels;
    }
}
