<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
use PHPUnit\Framework\TestCase;
use Hzw\Channel;
use Hzw\TaskManager;
use Hzw\Task\ChannelsData;
use Hzw\Task\ChannelsStatus;
use Hzw\Task\DeviceStatus;

class TaskManagerTest extends TestCase {
    /*
    function testBackupRestore() {
        $fn = "TaskManagerTest";

        $sn = '20180123000001';
        $seq = 1;
        $dt = mktime(8, 30, 0, 1, 23, 2018);
        $pcdt = mktime(12, 0, 0, 1, 23, 2018);

        $tasks[] = new ChannelsStatus($sn, $seq, $dt, $pcdt, $this->newChannels(4, 4, 0x20));
        $tasks[] = new ChannelsData($sn, $seq, $dt, $pcdt, $this->newChannels(4, 4, null, 1.23));
        $tasks[] = new DeviceStatus($sn, $seq, $dt, $pcdt, 256);
        $tasks[] = new ChannelsStatus($sn, $seq, $dt, $pcdt, $this->newChannels(4, 4, 0x00));
        $tasks[] = new ChannelsData($sn, $seq, $dt, $pcdt, $this->newChannels(4, 4, null, 3.21));
        $tasks[] = new DeviceStatus($sn, $seq, $dt, $pcdt, 259);

        $tasks[] = new DeviceStatus($sn, $seq, $dt, $pcdt, 259);
        $tasks[] = new ChannelsData($sn, $seq, $dt, $pcdt, $this->newChannels(4, 4, null, 3.21));
        $tasks[] = new ChannelsStatus($sn, $seq, $dt, $pcdt, $this->newChannels(4, 4, 0x00));
        $tasks[] = new DeviceStatus($sn, $seq, $dt, $pcdt, 256);
        $tasks[] = new ChannelsData($sn, $seq, $dt, $pcdt, $this->newChannels(4, 4, null, 1.23));
        $tasks[] = new ChannelsStatus($sn, $seq, $dt, $pcdt, $this->newChannels(4, 4, 0x20));

        foreach ($tasks as $task) {
            $task->backup($fn);
        }
        $this->assertEquals(true, file_exists($fn));

        $_tasks = TaskManager::restore($fn);
        $this->assertEquals($tasks, $_tasks);
        $this->assertEquals(false, file_exists($fn));
    }
    */

    function testRestoreFail() {
        $fn = "TaskManagerTest";

        $sn = '20180123000001';
        $seq = 1;
        $dt = mktime(8, 30, 0, 1, 23, 2018);
        $pcdt = mktime(12, 0, 0, 1, 23, 2018);

        $task = new ChannelsStatus($sn, $seq, $dt, $pcdt, $this->newChannels(4, 4, 0x20));
        $task->backup($fn);
        $task = new ChannelsData($sn, $seq, $dt, $pcdt, $this->newChannels(4, 4, null, 1.23));
        $task->backup($fn);
        ChannelsStatus::restore($fn);
        //ChannelsData::restore($fn);
        //TaskManager::restore($fn);
    }

    function newChannels($nlot, $nort, $status = null, $data = null) {
        $channels = [];

        for ($slot = 1; $slot < $nlot + 1; $slot++) {
            for ($port = 0; $port < $nort; $port++) {
                $type = Channel::WS;
                if ($slot < 3) {
                    $type = Channel::GND_L;
                } else if ($slot === 3) {
                    $type = Channel::GND_H;
                }

                $channel = new Channel($slot, $port, $type);

                if (is_numeric($status)) $channel->setStatus($status);
                else if (is_numeric($data)) $channel->setData($data);

                $channels[] = $channel;
            }
        }

        return $channels;
    }
}
