<?php
class Task {
    static $cntx;
    static $stmts = [];
    
    public static function init($cntx) {
        static::$cntx = static::class . "::$cntx";
    }

    public static function prepare($dbh) {
        foreach (static::$stmts as $stmt) {
            echo $stmt . PHP_EOL;
        }
    }
}

class ChannelsData extends Task {
    static $cntx;
}

class ChannelsStatus extends Task {
    static $cntx;
}

class TaskManager {
    private static $classes = [
        ChannelsData::class,
        ChannelsStatus::class,
    ];

    static function init($cntx) {
        foreach (static::$classes as $class) {
            $class::init($cntx);
        }
        //Task::init('hzw');
    }
}


TaskManager::init('hzw');
var_dump(Task::$cntx);
var_dump(ChannelsData::$cntx);
var_dump(ChannelsStatus::$cntx);

Task::prepare('haha');
