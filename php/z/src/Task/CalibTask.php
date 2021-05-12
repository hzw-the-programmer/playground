<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Task;

use Hzw\Task;
use Hzw\Channel;
use Hzw\Config;

class CalibTask extends Task {
    static $cntx;
    static $sqls = [
        'EXEC calib :sn, :slot, :port, :type, :dt, :pcdt, :seq, :result, :data',
    ];
    static $stmts = [];
    static $className;

    public $sn;
    public $seq;
    public $dt;
    public $pcdt;
    public $channels;

    public function __construct($sn, $seq, $dt, $pcdt, $channels) {
        $this->sn = $sn;
        $this->seq = $seq;
        $this->dt = $dt;
        $this->pcdt = $pcdt;
        $this->channels = $channels;
    }

    public function onTransaction($dbh) {
        $stmt = static::$stmts[0];

        $sn = $this->sn;
        $seq = $this->seq;
        $dt = $this->dt;
        $pcdt = $this->pcdt;
        $channels = $this->channels;

        $stmt->bindParam(':sn', $sn);
        $stmt->bindParam(":seq", $seq);
        $stmt->bindParam(":dt", $dt);
        $stmt->bindParam(":pcdt", $pcdt);

        foreach ($channels as $channel) {
            $slot = $channel->getSlot();
            $port = $channel->getPort();
            $type = $channel->getType();
            $result = $channel->getCalibResult();
            $data = $channel->getCalibData();

            $stmt->bindParam(':slot', $slot);
            $stmt->bindParam(':port', $port);
            $stmt->bindParam(':type', $type);
            $stmt->bindParam(":result", $result);
            $stmt->bindParam(":data", $data);

            $stmt->execute();
        }
    }
}
