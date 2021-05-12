<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Task;

use Hzw\Task;

class DeviceStandbyHBTask extends Task {
    static $cntx;
    static $sqls = [
        '
            EXEC deviceStandbyHB :sn, :dt
        ',
        '
            UPDATE ds
            SET ds.rssi = :signal,
                ds.updatetime = :dt
            FROM device_rssi AS ds
            LEFT JOIN devices_info AS di
            ON ds.device_id = di.id
            WHERE di.sn = :sn
        '
    ];
    static $stmts = [];
    static $className;

    public $sn;
    public $dt;

    public $signal;

    function __construct($sn, $dt, $signal) {
        $this->sn = $sn;
        $this->dt = $dt;

        $this->signal = $signal;
    }

    function onTransaction($dbh) {
        $stmt = static::$stmts[0];

        $sn = $this->sn;
        $dt = $this->dt;

        $stmt->bindParam(':sn', $sn);
        $stmt->bindParam(":dt", $dt);

        $stmt->execute();

        $stmt = static::$stmts[1];

        $stmt->bindParam(':signal', $this->signal);
        $stmt->bindParam(":dt", $dt);
        $stmt->bindParam(":sn", $sn);

        $stmt->execute();
    }
}
