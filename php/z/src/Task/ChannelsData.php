<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Task;

use Hzw\Task;
use Hzw\Channel;
use Hzw\Config;

class ChannelsData extends Task {
    static $cntx;
    static $sqls = [
        'EXEC new_channel_data :sn, :slot, :port, :type, :dt, :seq, :pcdt, :data',
    ];
    static $stmts = [];
    static $className;

    public $sn;
    public $seq;
    public $dt;
    public $pcdt;
    public $channels;

    function __construct($sn, $seq, $dt, $pcdt, $channels) {
        $this->sn = $sn;
        $this->seq = $seq;
        $this->dt = $dt;
        $this->pcdt = $pcdt;
        $this->channels = $channels;
    }

    function onTransaction($dbh) {
        $logger = static::$cntx->getLogger();

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
            $data = $channel->getData();

            $stmt->bindParam(':slot', $slot);
            $stmt->bindParam(':port', $port);
            $stmt->bindParam(':type', $type);
            $stmt->bindParam(":data", $data);

            $stmt->execute();
        }
    }

    function onFail($result) {
        $workerId = static::$cntx->getServ()->worker_id;
        $this->backup(Config::DATA_DIR . "/data$workerId");
    }

    function onBackup($f) {
        $className = static::className();

        $sn = $this->sn;
        $seq = $this->seq;
        $dt = $this->dt;
        $pcdt = $this->pcdt;
        $channels = $this->channels;

        fputcsv($f, [$className, $sn, $seq, $dt, $pcdt]);
        foreach ($channels as $channel) {
            fputcsv($f, [$channel->getSlot(), $channel->getPort(), $channel->getType(), $channel->getData()]);
        }
        fputcsv($f, [$className]);
    }

    static function onRestore($f) {
        $className = static::className();

        $fields = fgetcsv($f);
        if (!$fields || $fields[0] !== $className) {
            return null;
        }

        list(, $sn, $seq, $dt, $pcdt) = $fields;

        $channels = [];
        while (($fields = fgetcsv($f)) && $fields[0] !== $className) {
            list($slot, $port, $type, $data) = $fields;
            
            $channel = new Channel($slot, $port, $type);
            $channel->setData($data);

            $channels[] = $channel;
        }

        return new static($sn, $seq, $dt, $pcdt, $channels);
    }
}
