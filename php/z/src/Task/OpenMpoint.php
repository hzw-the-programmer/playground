<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Task;

use Hzw\Task;
use Hzw\DatabaseManager;
use Hzw\Cmd\Open;

class OpenMpoint extends Task {
    static $cntx;
    static $className;
    static $sqls = [
        'EXEC :mid = openMpoint :sn, :slot, :port, :type, :pid, :name, :dt',
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
            $stmt->bindParam(':sn', $sn);
            $stmt->bindParam(":dt", $dt);

            foreach ($channels as $channel) {
                $mid = 0;
                $slot = $channel->getSlot();
                $port = $channel->getPort();
                $type = $channel->getType();
                $pid = $channel->getPid();
                $name = $channel->getName();
    
                $stmt->bindParam(":mid", $mid, \PDO::PARAM_INT, 1);
                $stmt->bindParam(":slot", $slot);
                $stmt->bindParam(":port", $port);
                $stmt->bindParam(":type", $type);
                $stmt->bindParam(":pid", $pid);
                $stmt->bindParam(":name", $name);
    
                //$stmt->execute();

                $channel = clone $channel;
                $channel->setMid($mid);
                $device->removeChannel($channel)
                       ->addChannel($channel);
            }

            if ($serv->exist($fd)) {
                $cntx->push($fd, json_encode(['sn' => $this->sn, 'cmd' => Open::CMD, 'result' => 0]));
            }
            $cntx->enable();
            return self::SUCCESS;
        } catch (\Exception $e) {
            $logger->critical('OpenMpoint fail: ' . $e->getMessage());
            $databaseMgr->disconnect();
        }

        fail:
        if (!$this->subTask) {
            $serv->after(1000, [$this, 'process']);
        }
        return self::FAIL;
    }
}
