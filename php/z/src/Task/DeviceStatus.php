<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Task;

use Hzw\Task;
use Hzw\Device;
use Hzw\Config;

class DeviceStatus extends Task {
    static $cntx;
    static $sqls = [
        'EXEC new_device_status :sn, :dt, :status',
    ];
    static $stmts = [];
    static $className;

    public $sn;
    public $seq;
    public $dt;
    public $pcdt;
    public $status;

    function __construct($sn, $seq, $dt, $pcdt, $status) {
        $this->sn = $sn;
        $this->seq = $seq;
        $this->dt = $dt;
        $this->pcdt = $pcdt;
        $this->status = $status;
    }

    function getStatus() {
        return $this->status;
    }

    function onTransaction($dbh) {
        $logger = static::$cntx->getLogger();
        
        $stmt = static::$stmts[0];
        
        $sn = $this->sn;
        $seq = $this->seq;
        $dt = $this->dt;
        $pcdt = $this->pcdt;
        $status = $this->status;

        $stmt->bindParam(":sn", $sn);
        $stmt->bindParam(":dt", $dt);
        $stmt->bindParam(":status", $status);

        $stmt->execute();
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
        $status = $this->status;

        fputcsv($f, [$className, $sn, $seq, $dt, $pcdt, $status]);
    }

    static function onRestore($f) {
        $className = static::className();

        $fields = fgetcsv($f);
        if (!$fields || $fields[0] !== $className) {
            return null;
        }

        list(, $sn, $seq, $dt, $pcdt, $status) = $fields;

        return new static($sn, $seq, $dt, $pcdt, $status);
    }
}
