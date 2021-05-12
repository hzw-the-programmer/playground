<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Task;

use Hzw\Task;
use Hzw\Device;
use Hzw\Channel;
use Hzw\DatabaseManager;

class LoadDevices extends Task {
    static $cntx;
    static $className;
    static $sqls = [
        '
            SELECT
                di.id AS did, di.sn, dip.sn AS psn, di.ip, di.port, di.devaddr, di.type,
                ci.id AS cid, ci.slot, ci.port AS cport, ci.type AS ctype,
                mp.id AS mid, mp.pid, mp.name,
                mprs.raw_status AS status, mprs.time AS dt, mprs.swiftnum AS seq,
                mpps.raw_status AS preStatus, mpps.time AS preDt, mpps.swiftnum AS preSeq
            FROM devices_info AS di
            LEFT JOIN channels_info AS ci
            ON di.id = ci.device_id
            LEFT JOIN mpoint AS mp
            ON ci.id = mp.ciid AND mp.endtime = 0
            LEFT JOIN mpoint_realtime_status AS mprs
            ON mp.id = mprs.mpoint_id
            LEFT JOIN mpoint_pre_status AS mpps
            ON mp.id = mpps.mpoint_id
            LEFT JOIN devices_info AS dip
            ON di.parent_id = dip.id
        '
    ];
    static $stmts = [];

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

        try {
            $stmt->execute();
            $devices = [];
            while ($row = $stmt->fetch(\PDO::FETCH_ASSOC)) {
                $sn = $row['sn'];
                if (!isset($devices[$sn])) {
                    $devices[$sn] = Device::createFromDb($row);
                }
                $device = $devices[$sn];

                if ($row['mid'] && !$device->getChannel($row['slot'], $row['cport'])) {
                    $device->addChannel(Channel::createFromDb($row));
                }
            }
            $deviceMgr->setDevices($devices);
            $cntx->enable();
            return self::SUCCESS;
        } catch (\Exception $e) {
            $logger->critical('LoadDevices fail: ' . $e->getMessage());
            $databaseMgr->disconnect();
        }

        fail:
        if (!$this->subTask) {
            $serv->after(1000, [$this, 'process']);
        }
        return self::FAIL;
    }
}
