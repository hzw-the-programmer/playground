<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Task;

use Hzw\Task;

class DbConnect extends Task {
    static $cntx;
    static $sqls = [];
    static $stmts = [];
    static $className;

    function process() {
        $databaseMgr = self::$cntx->getDatabaseMgr();
        return $databaseMgr->connect();
    }
}
