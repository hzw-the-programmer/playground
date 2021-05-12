<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

class Task {
    const HEADER_SIZE = 10 + 1;
    const HEADER_FORMAT = "%010d";

    const SUCCESS = 0;
    const FAIL = 1;

    static $cntx;
    static $className;
    static $sqls = [];
    static $stmts = [];

    protected $subTask = false;

    static function init($cntx) {
        static::$cntx = $cntx;

        $className = static::class;
        static::$className = substr($className, strrpos($className, '\\') + 1);
    }

    static function prepare($dbh) {
        if ($dbh) {
            foreach (static::$sqls as $sql) {
                static::$stmts[] = $dbh->prepare($sql);
            }
        } else {
            static::$stmts = [];
        }
    }

    static function className() {
        return static::$className;
    }

    function setSubTask($subTask) {
        $this->subTask = $subTask;
        return $this;
    }

    function isSubTask() {
        return $this->subTask;
    }

    function __destruct() {
        $logger = static::$cntx->getLogger();
        $logger->debug(static::$className . ' destruct.');
    }

    function process() {
        $databaseMgr = static::$cntx->getDatabaseMgr();

        $result = $databaseMgr->transaction([$this, 'onTransaction']);
        
        if ($result === DatabaseManager::SUCCESS) {
            $this->onSucc();
        } else {
            $this->onFail($result);
        }

        return $result;
    }

    function onTransaction($dbh) {
    }

    function onFail($result) {
    }

    function onSucc() {
    }

    function backup($fn) {
        $f = fopen($fn, 'c');
        fseek($f, 0, SEEK_END);
        if (ftell($f) === 0) {
            fputcsv($f, [sprintf(Task::HEADER_FORMAT, Task::HEADER_SIZE)]);
        }
        $this->onBackup($f);
        fclose($f);
    }

    function onBackup($f) {
    }

    static function restore($fn) {
        $f = @fopen($fn, 'r+');
        if (!$f) return null;

        $fields = fscanf($f, Task::HEADER_FORMAT);
        if (!$fields) return null;

        list($pos) = $fields;
        if (!is_numeric($pos) || $pos < Task::HEADER_SIZE) return null;

        fseek($f, 0, SEEK_END);
        $size = ftell($f);
        if ($pos >= $size) return null;

        fseek($f, $pos);

        $task = static::onRestore($f);

        if ($task) {
            $pos = ftell($f);
            fseek($f, 0);
            fputcsv($f, [sprintf(Task::HEADER_FORMAT, $pos)]);
        }

        fclose($f);

        if ($pos === $size) {
            unlink($fn);
        }

        return $task;
    }

    static function onRestore($f) {
        return null;
    }

    function __toString() {
        $str = '{';
        $str .= "name:" . static::className() . ",";
        foreach ($this as $key => $value) {
            if ($key === 'dt' || $key === 'pcdt') {
                $value = date('Y-m-d H:i:s', $value);
            } else if ($key === 'channels') {
                $channels = $value;
                $value = '[';
                foreach ($channels as $channel) {
                    $value .= $channel . ',';
                }
                $value = rtrim($value, ',');
                $value .= ']';
            } else if ($key === 'status') {
                $value = Device::$statuses[$value];
            } else if ($key === 'address') {
                $value = implode(':', $value);
            }

            $str .= "$key:$value,";
        }
        $str = rtrim($str, ',');
        $str .= '}';

        return $str;
    }
}
