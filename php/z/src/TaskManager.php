<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

use Hzw\Cmd\SetSn;
use Hzw\Cmd\GetSn;
use Hzw\Cmd\SetWorkTime;
use Hzw\Cmd\GetWorkTime;
use Hzw\Cmd\Calibration;
use Hzw\Cmd\DeviceStandbyHB;
use Hzw\Cmd\Ack;
use Hzw\Cmd\StatusData;
use Hzw\Cmd\Data;
use Hzw\Cmd\PowerOff;
use Hzw\Cmd\PowerOn;
use Hzw\Cmd\DeviceInfo;
use Hzw\Cmd\HeartBeatWithData;
use Hzw\Cmd\Open;
use Hzw\Cmd\Close;
use Hzw\Cmd\Status;
use Hzw\Cmd\GetParam;
use Hzw\Cmd\SetParam;
use Hzw\Cmd\HeartBeat;

use Hzw\Task\CalibTask;
use Hzw\Task\ChannelsData;
use Hzw\Task\ChannelsStatus;
use Hzw\Task\CloseMpoint;
use Hzw\Task\DataRestore;
use Hzw\Task\DbConnect;
use Hzw\Task\DeviceSignal;
use Hzw\Task\DeviceStandbyHBTask;
use Hzw\Task\DeviceStatus;
use Hzw\Task\DevInfo;
use Hzw\Task\HBUpdateAvgData;
use Hzw\Task\HBUpdateStatusSignal;
use Hzw\Task\HBWithData;
use Hzw\Task\LoadDevices;
use Hzw\Task\OfflineDevices;
use Hzw\Task\OpenMpoint;
use Hzw\Task\StatusDataTask;
use Hzw\Task\Tasks;
use Hzw\Task\UpdateDevice;

class TaskManager {
    const IDLE_TASK_WORKER = -1;
    const ALL_TASK_WORKERS = -2;

    private static $cntx;
    private static $classes = [
        CalibTask::class,
        ChannelsData::class,
        ChannelsStatus::class,
        CloseMpoint::class,
        DataRestore::class,
        DbConnect::class,
        DeviceSignal::class,
        DeviceStandbyHBTask::class,
        DeviceStatus::class,
        DevInfo::class,
        HBUpdateAvgData::class,
        HBUpdateStatusSignal::class,
        HBWithData::class,
        LoadDevices::class,
        OfflineDevices::class,
        OpenMpoint::class,
        StatusDataTask::class,
        Tasks::class,
        UpdateDevice::class,
    ];

    static function init($cntx) {
        static::$cntx = $cntx;

        foreach (static::$classes as $class) {
            $class::init($cntx);
        }
    }

    static function prepare($dbh) {
        foreach (static::$classes as $class) {
            $class::prepare($dbh);
        }
    }

    static function onDbConnected($dbh) {
        static::prepare($dbh);
    }

    static function onDbDisConnected($dbh) {
        static::prepare(null);
    }

    static function onPacket($packet) {
        $header = $packet->getHeader();
        $body = $packet->getBody();
        $pcdt = $packet->getPcdt();

        $sns = $header->getSns();
        $seq = $header->getSeq();
        $dt = $header->getDt();

        $sn = $sns[count($sns) - 1];

        switch ($body::CMD) {
            case Status::CMD:
            case StatusData::CMD:
                if ($packet->fromLora()) {
                    self::realtimeTask(new StatusDataTask(
                        $sn, $seq, $dt, $pcdt, $body->getChannels()
                    ));
                } else {
                    self::realtimeTask(new ChannelsStatus(
                        $sn, $seq, $dt, $pcdt, $body->getChannels()
                    ));
                }
                break;

            case Data::CMD:
                self::normalTask(new ChannelsData(
                    $sn, $seq, $dt, $pcdt, $body->getChannels()
                ));
                if ($extraInfo = $packet->getExtraInfo()) {
                    $signal = $extraInfo->rssi;
                    self::normalTask(new DeviceSignal(
                        $sn, $seq, $dt, $pcdt, $signal
                    ));
                }
                break;

            case PowerOn::CMD:
                $status = Device::POWERON;
            case PowerOff::CMD:
                if (!isset($status)) {
                    $dt = $body->getDt();
                    $status = Device::POWEROFF;
                }
                self::realtimeTask(new DeviceStatus(
                    $sn, $seq, $dt, $pcdt, $status
                ));
                break;

            case HeartBeat::CMD:
                $signal = $body->getSignal();
                if ($extraInfo = $packet->getExtraInfo()) {
                    $signal = $extraInfo->rssi;
                }
                self::normalTask(new DeviceSignal(
                    $sn, $seq, $dt, $pcdt, $signal
                ));
                break;

            case DeviceInfo::CMD:
                self::normalTask(new DevInfo(
                    $sn, $body->getVersion()
                ));
                break;

            case HeartBeatWithData::CMD:
            case Calibration::CMD:
            case DeviceStandbyHB::CMD:
                $body->process(self::$cntx, $packet);
                break;

            default:
                break;
        }
    }

    static function realtimeTask($task) {
        $cntx = self::$cntx;
        $serv = $cntx->getServ();

        $workerNum = $serv->setting['worker_num'];
        self::task($task, $workerNum);
    }

    static function normalTask($task) {
        $cntx = self::$cntx;
        $serv = $cntx->getServ();

        $workerNum = $serv->setting['worker_num'];
        self::task($task, $workerNum + 1);
    }

    static function workerTask($task) {
        $cntx = self::$cntx;
        $serv = $cntx->getServ();

        self::task($task, $serv->worker_id);
    }

    static function task($task, $workerId = self::IDLE_TASK_WORKER) {
        $cntx = self::$cntx;
        $serv = $cntx->getServ();
        $logger = $cntx->getLogger();

        $workerNum = $serv->setting['worker_num'];
        $taskWorkerNum = $serv->setting['task_worker_num'];

        if ($workerId === self::IDLE_TASK_WORKER) {
            // do nothing
        } else if ($workerId === self::ALL_TASK_WORKERS) {
            for ($i = 0; $i < $taskWorkerNum; $i++) {
                self::task($task, $workerNum + $i);
            }
            return;
        } else if ($workerId < $workerNum) {
            self::onFinish($serv, -1, self::onTask($serv, -1, $workerId, $task));
            return;
        } else {
            $workerId -= $workerNum;
        }

        if ($serv->task($task, $workerId) === false) {
            $logger->critical("Task {$task::className()} deliver fail.");
        }
    }

    static function onTask($serv, $taskId, $srcWorkerId, $task) {
        $cntx = self::$cntx;
        $serv = $cntx->getServ();
        $logger = $cntx->getLogger();

        list($result, $elapsed) = Debug::measure([$task, 'process']);
        $logger->debug("Task {$task::className()} cost: $elapsed");

        return [$serv->worker_id, $result];
    }

    static function onFinish($serv, $taskId, $data) {
        list($workerId, $result) = $data;

        if ($result === Task::SUCCESS
            || $result === Task::FAIL
        ) {
            return;
        }

        switch ($result) {
            case DatabaseManager::DISCONNECTED:
                $serv->after(Config::DB_CONNECT_INTERVAL, function() use ($workerId) {
                    self::task(new DbConnect(), $workerId);
                });
                break;

            case DatabaseManager::CONNECTED:
            case DataRestore::CONTINUE:
                $serv->after(Config::DATA_RESTORE_INTERVAL, function() use ($workerId) {
                    self::task(new DataRestore(), $workerId);
                });
                break;

            default:
                break;
        }
    }

    static function restore($fn) {
        $logger = static::$cntx->getLogger();

        $fexists = file_exists($fn);
        if (!$fexists) {
            $logger->critical('data restore: finish: no data');
            return DataRestore::FINISH;
        }

        $batch = 10;
        while ($fexists && $batch > 0) {
            foreach (static::$classes as $class) {
                if ($task = $class::restore($fn)) {
                    $result = $task->process();
                    if ($result === DatabaseManager::SUCCESS) {
                        break;
                    } else if ($result === DatabaseManager::FAIL) {
                        $logger->critical('data restore: continue: db fail');
                        return DataRestore::CONTINUE;
                    } else {
                        $logger->critical("data restore: $result");
                        return $result;
                    }
                }
            }

            $fexists = file_exists($fn);
            $batch--;
        }

        if ($fexists) {
            $logger->critical('data restore: continue: more data');
            return DataRestore::CONTINUE;
        } else {
            $logger->critical('data restore: finish: all processed');
            return DataRestore::FINISH;
        }
    }
}
