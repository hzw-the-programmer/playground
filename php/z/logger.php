<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
include_once "./config.php";

class Logger {
    public function log($time, $rw, $clientInfo, $data) {
        if (!file_exists(LOG_DIR)) {
            mkdir(LOG_DIR);
        }
        $logFile = LOG_DIR . "/" . date("Y-m-d") . ".log";
        $line = sprintf("%s,%s,%-15s,%s,%s\n",
            date("Y-m-d H:i:s", $time),
            $rw,
            $clientInfo["address"],
            $clientInfo["port"],
            bin2hex($data));
        file_put_contents($logFile, $line, FILE_APPEND);
    }
}
