<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Task;

use Hzw\Task;
use Hzw\Config;
use Hzw\DatabaseManager;

class OfflineDevices extends Task {
    static $cntx;
    static $className;
    static $sqls = [
        'EXEC offlineDevices :dt',
    ];
    static $stmts = [];

    function process() {
        $cntx = self::$cntx;
        $serv = $cntx->getServ();
        $databaseMgr = $cntx->getDatabaseMgr();
        $logger = $cntx->getLogger();

        $dt = @fileatime(Config::LOG_DIR . '/touch');
        if (!$dt) {
            return self::SUCCESS;
        }
        $dt += Config::OFFLINE_TIMEOUT;

        $cntx->disable();

        if ($databaseMgr->connect() === DatabaseManager::DISCONNECTED) {
            goto fail;
        }
        $stmt = self::$stmts[0];

        try {
            $stmt->bindParam('dt', $dt);
            $stmt->execute();
            $cntx->enable();
            return self::SUCCESS;
        } catch (\Exception $e) {
            $logger->critical('OfflineDevices fail: ' . $e->getMessage());
            $databaseMgr->disconnect();
        }

        fail:
        if (!$this->subTask) {
            $serv->after(1000, [$this, 'process']);
        }
        return self::FAIL;
    }
}
