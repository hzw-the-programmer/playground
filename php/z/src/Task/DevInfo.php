<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Task;

use Hzw\Task;

class DevInfo extends Task {
	static $cntx;
	static $className;
    static $sqls = [
        'UPDATE devices_info SET version = :version, updatetime = :dt WHERE sn = :sn'
    ];
    static $stmts = [];

    private $sn;
    private $version;

    public function __construct($sn, $version) {
        $this->sn = $sn;
        $this->version = $version;
    }

    public function onTransaction($dbh) {
        $stmt = static::$stmts[0];

        $dt = time();
        $stmt->bindParam(":version", $this->version);
        $stmt->bindParam(":dt", $dt);
        $stmt->bindParam(":sn", $this->sn);

        $stmt->execute();
    }
}
