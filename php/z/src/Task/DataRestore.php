<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Task;

use Hzw\Task;
use Hzw\TaskManager;
use Hzw\Config;

class DataRestore extends Task {
    const CONTINUE = 'DataRestore::CONTINUE';
    const FINISH = 'DataRestore::FINISH';

    static $cntx;
    static $sqls = [];
    static $stmts = [];
    static $className;

    function process() {
        $workerId = static::$cntx->getServ()->worker_id;
        return TaskManager::restore(Config::DATA_DIR . "/data$workerId");
    }
}
