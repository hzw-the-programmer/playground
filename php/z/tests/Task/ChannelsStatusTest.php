<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
use PHPUnit\Framework\TestCase;
use Hzw\Task\ChannelsStatus;
use Hzw\Channel;

class ChannelsStatusTest extends TestCase {
    function testRestore() {
        $fn = ChannelsStatus::className();

        $this->assertEquals(null, ChannelsStatus::restore($fn));

        touch($fn);
        $this->assertEquals(null, ChannelsStatus::restore($fn));
        unlink($fn);

        $contents = '';
        file_put_contents($fn, $contents);
        $this->assertEquals(null, ChannelsStatus::restore($fn));
        unlink($fn);

        $contents = 'abcd';
        file_put_contents($fn, $contents);
        $this->assertEquals(null, ChannelsStatus::restore($fn));
        unlink($fn);

        $contents = '9abcd123';
        file_put_contents($fn, $contents);
        $this->assertEquals(null, ChannelsStatus::restore($fn));
        unlink($fn);

        $contents = '11a';
        file_put_contents($fn, $contents);
        $this->assertEquals(null, ChannelsStatus::restore($fn));
        unlink($fn);
    }

    function testBackupRestore() {
        $sn = '20180123000001';
        $seq = 1;
        $dt = mktime(8, 30, 0, 1, 23, 2018);
        $pcdt = mktime(12, 0, 0, 1, 23, 2018);

        $channels = [];
        for ($slot = 1; $slot < 5; $slot++) {
            for ($port = 0; $port < 4; $port++) {
                $type = Channel::WS;
                if ($slot < 3) {
                    $type = Channel::GND_L;
                } else if ($slot === 3) {
                    $type = Channel::GND_H;
                }
                $channel = new Channel($slot, $port, $type);
                $channel->setStatus(0x20);
                $channels[] = $channel;
            }
        }
        
        $task = new ChannelsStatus($sn, $seq, $dt, $pcdt, $channels);
        
        $contents = "0000000011\n";
        $contents .= "ChannelsStatus,20180123000001,1,1516696200,1516708800\n";
        $contents .= "1,0,12,32\n";
        $contents .= "1,1,12,32\n";
        $contents .= "1,2,12,32\n";
        $contents .= "1,3,12,32\n";
        $contents .= "2,0,12,32\n";
        $contents .= "2,1,12,32\n";
        $contents .= "2,2,12,32\n";
        $contents .= "2,3,12,32\n";
        $contents .= "3,0,8,32\n";
        $contents .= "3,1,8,32\n";
        $contents .= "3,2,8,32\n";
        $contents .= "3,3,8,32\n";
        $contents .= "4,0,9,32\n";
        $contents .= "4,1,9,32\n";
        $contents .= "4,2,9,32\n";
        $contents .= "4,3,9,32\n";
        $contents .= "ChannelsStatus\n";

        $fn = ChannelsStatus::className();

        $task->backup($fn);
        $_contents = file_get_contents($fn);
        $this->assertEquals($contents, $_contents);

        $this->assertEquals(true, file_exists($fn));
        $_task = ChannelsStatus::restore($fn);
        $this->assertEquals($task, $_task);
        $this->assertEquals(false, file_exists($fn));
    }
}
