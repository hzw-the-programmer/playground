<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Cmd;

class Cmd {
    const SetSn = 0x05;
    const GetSn = 0x06;
    const SetWorkTime = 0x07;
    const GetWorkTime = 0x08;
    const Calibration = 0x09;
    const DeviceStandbyHB = 0x0A;
    const Ack = 0x24;
    const StatusData = 0x40;
    const Data = 0x41;
    const PowerOff = 0x42;
    const PowerOn = 0x43;
    const DeviceInfo = 0x44;
    const HeartBeatWithData = 0x45;
    const Open = 0x49;
    const Close = 0x50;
    const Status = 0x51;
    const GetParam = 0x52;
    const SetParam = 0x53;
    const HeartBeat = 0xFE;

    function __toString() {
        $str = '{';
        $str .= substr(static::class, strrpos(static::class, '\\') + 1);
        $str .= '}';
        return $str;
    }
}
