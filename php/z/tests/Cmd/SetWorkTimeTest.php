<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
use PHPUnit\Framework\TestCase;
use Hzw\Cmd\SetWorkTime;
use Hzw\Cmd\Ack;

class SetWorkTimeTest extends TestCase
{
    function testPack()
    {
        $periods = [
            [mktime(8, 0), mktime(12, 0)],
            [mktime(13, 0), mktime(17, 0)],
            [mktime(18, 0), mktime(20, 0)]
        ];
        
        $cmd = new SetWorkTime($periods);

        $this->assertEquals(
            '070308000c000d00110012001400',
            bin2hex($cmd->pack())
        );
    }

    function testUnpack()
    {
        $cmd = SetWorkTime::unpack(hex2bin('070308000c000d00110012001400'));
        
        $periods = $cmd->getPeriods();
        
        $lt = localtime($periods[0][0], true);
        $this->assertEquals(8, $lt['tm_hour']);
        $this->assertEquals(0, $lt['tm_min']);

        $lt = localtime($periods[0][1], true);
        $this->assertEquals(12, $lt['tm_hour']);
        $this->assertEquals(0, $lt['tm_min']);

        $lt = localtime($periods[1][0], true);
        $this->assertEquals(13, $lt['tm_hour']);
        $this->assertEquals(0, $lt['tm_min']);

        $lt = localtime($periods[1][1], true);
        $this->assertEquals(17, $lt['tm_hour']);
        $this->assertEquals(0, $lt['tm_min']);

        $lt = localtime($periods[2][0], true);
        $this->assertEquals(18, $lt['tm_hour']);
        $this->assertEquals(0, $lt['tm_min']);

        $lt = localtime($periods[2][1], true);
        $this->assertEquals(20, $lt['tm_hour']);
        $this->assertEquals(0, $lt['tm_min']);
    }

    function testUnpackNull()
    {
        $cmd = SetWorkTime::unpack(hex2bin('080308000c000d00110012001400'));

        $this->assertEquals(null, $cmd);
    }

    function testPackAck()
    {
        $cmd = new Ack(0x12345678, 0x00, SetWorkTime::CMD, null);

        $this->assertEquals(
            '24123456780007',
            bin2hex($cmd->pack())
        );
    }

    function testUnpackAck()
    {
        $cmd = Ack::unpack(hex2bin('24123456780007'));

        $this->assertEquals(0x12345678, $cmd->getSeq());
        $this->assertEquals(0x00, $cmd->getStatus());
        $this->assertEquals(SetWorkTime::CMD, $cmd->getCmd());
        $this->assertEquals(null, $cmd->getData());
    }
}
