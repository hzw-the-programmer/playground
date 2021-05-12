<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

class Param {
    public $type;
    public $analogInterval;
    public $lowerLimit;
    public $upperLimit;
    public $alarmDelay;
    public $level1Delay;
    public $level2Delay;
    public $level3Delay;

    function __construct($type, $analogInterval, $lowerLimit, $upperLimit,
        $alarmDelay, $level1Delay, $level2Delay, $level3Delay) {
        $this->type = $type;
        $this->analogInterval= $analogInterval;
        $this->lowerLimit = $lowerLimit;
        $this->upperLimit = $upperLimit;
        $this->alarmDelay = $alarmDelay;
        $this->level1Delay = $level1Delay;
        $this->level2Delay = $level2Delay;
        $this->level3Delay = $level3Delay;
    }

    function getType() {
        return $this->type;
    }
}
