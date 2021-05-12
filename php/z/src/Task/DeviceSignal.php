<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Task;

use Hzw\Task;

class DeviceSignal extends Task {
    static $cntx;
    static $sqls = [
        'EXEC deviceSignal :sn, :signal, :dt',
    ];
    static $stmts = [];
    static $className;

    public $sn;
    public $seq;
    public $dt;
    public $pcdt;
    public $signal;

    function __construct($sn, $seq, $dt, $pcdt, $signal) {
        $this->sn = $sn;
        $this->seq = $seq;
        $this->dt = $dt;
        $this->pcdt = $pcdt;
        $this->signal = $signal;
    }

    function onTransaction($dbh) {
        $logger = static::$cntx->getLogger();

        $stmt = static::$stmts[0];

        $sn = $this->sn;
        $seq = $this->seq;
        $dt = $this->dt;
        $pcdt = $this->pcdt;
        $signal = $this->signal;

        $stmt->bindParam(":sn", $sn);
        $stmt->bindParam(":dt", $dt);
        $stmt->bindParam(":signal", $signal);

        $stmt->execute();
    }
}
