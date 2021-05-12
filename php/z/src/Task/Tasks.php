<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Task;

use Hzw\Task;

class Tasks extends Task {
    static $cntx;
    static $className;
    static $sqls = [];
    static $stmts = [];

    private $tasks;

    function __construct($tasks) {
        foreach($tasks as $task) {
            $task->setSubTask(true);
        }
        $this->tasks = $tasks;
    }

    function process() {
        $cntx = self::$cntx;
        $serv = $cntx->getServ();

        foreach ($this->tasks as $task) {
            if ($task->process() === self::FAIL) {
                $serv->after(1000, [$this, 'process']);
                return self::FAIL;
            }
        }

        return self::SUCCESS;
    }
}
