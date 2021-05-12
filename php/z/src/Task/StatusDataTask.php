<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Task;

use Hzw\Task;
use Hzw\Channel;
use Hzw\Config;

class StatusDataTask extends Task {
    static $cntx;
    static $sqls = [
        'EXEC statusData :sn, :slot, :port, :type, :dt, :pcdt, :seq, :status, :realStatus, :level, :data',
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
            $status = $channel->getStatus();
            $realStatus = $channel->getRealStatus();
            $level = $channel->getLevel();
            $data = $channel->getData();

            $stmt->bindParam(':slot', $slot);
            $stmt->bindParam(':port', $port);
            $stmt->bindParam(':type', $type);
            $stmt->bindParam(":status", $status);
            $stmt->bindParam(":realStatus", $realStatus);
            $stmt->bindParam(":level", $level);
            $stmt->bindParam(":data", $data);

            $stmt->execute();
        }
    }
}
