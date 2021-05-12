<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Task;

use Hzw\Task;
use Hzw\DatabaseManager;
use Hzw\Cmd\Close;

class CloseMpoint extends Task {
    static $cntx;
    static $className;
    static $sqls = [
        'EXEC closeMpoint :mid, :dt',
    ];
    static $stmts = [];

    public $sn;
    public $seq;
    public $dt;
    public $pcdt;
    public $channels;
    public $fd;

    function __construct($sn, $seq, $dt, $pcdt, $channels, $fd) {
        $this->sn = $sn;
        $this->seq = $seq;
        $this->dt = $dt;
        $this->pcdt = $pcdt;
        $this->channels = $channels;
        $this->fd = $fd;
    }

    function process() {
        $cntx = self::$cntx;
        $serv = $cntx->getServ();
        $databaseMgr = $cntx->getDatabaseMgr();
        $deviceMgr = $cntx->getDeviceMgr();
        $logger = $cntx->getLogger();

        $cntx->disable();

        if ($databaseMgr->connect() === DatabaseManager::DISCONNECTED) {
            goto fail;
        }
        $stmt = self::$stmts[0];

        $sn = $this->sn;
        $seq = $this->seq;
        $dt = $this->dt;
        $pcdt = $this->pcdt;
        $channels = $this->channels;
        $fd = $this->fd;

        $device = $deviceMgr->getDevice($sn);

        try {
            $stmt->bindParam(":dt", $dt);

            foreach ($channels as $channel) {
                $slot = $channel->getSlot();
                $port = $channel->getPort();
                $channel = $device->getChannel($slot, $port);
                if (!$channel) {
                    $logger->critical(self::$className . ": $sn $slot $port does not not exist.");
                    continue;
                }
                $mid = $channel->getMid();
    
                $stmt->bindParam(":mid", $mid);

                $stmt->execute();

                $device->removeChannel($channel);
            }

            if ($serv->exist($fd)) {
                $cntx->push($fd, json_encode(['sn' => $this->sn, 'cmd' => Close::CMD, 'result' => 0]));
            }
            $cntx->enable();
            return self::SUCCESS;
        } catch (\Exception $e) {
            $logger->critical('CloseMpoint fail: ' . $e->getMessage());
            $databaseMgr->disconnect();
        }

        fail:
        if (!$this->subTask) {
            $serv->after(1000, [$this, 'process']);
        }
        return self::FAIL;
    }
}
