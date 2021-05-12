<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
use PHPUnit\Framework\TestCase;
use Hzw\Cmd\GetWorkTime;
use Hzw\Cmd\Ack;

class GetWorkTimeTest extends TestCase
{
    function testPack()
    {
        $cmd = new GetWorkTime;

        $this->assertEquals(
            '08',
            bin2hex($cmd->pack())
        );
    }

    function testUnpack()
    {
        $cmd = GetWorkTime::unpack(hex2bin('08'));
 
        $this->assertEquals(GetWorkTime::CMD, $cmd::CMD);
    }

    function testUnpackNull()
    {
        $cmd = GetWorkTime::unpack(hex2bin('07'));

        $this->assertEquals(null, $cmd);
    }

    function testPackAck()
    {
        $periods = [
            [mktime(8, 0), mktime(12, 0)],
            [mktime(13, 0), mktime(17, 0)],
            [mktime(18, 0), mktime(20, 0)]
        ];

        $cmd = new Ack(0x12345678, 0x00, GetWorkTime::CMD, $periods);

        $this->assertEquals(
            '241234567800080308000c000d00110012001400',
            bin2hex($cmd->pack())
        );
    }

    function testUnpackAck()
    {
        $cmd = Ack::unpack(hex2bin('241234567800080308000c000d00110012001400'));

        $this->assertEquals(0x12345678, $cmd->getSeq());
        $this->assertEquals(0x00, $cmd->getStatus());
        $this->assertEquals(GetWorkTime::CMD, $cmd->getCmd());
        
        $periods = $cmd->getData();
        
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
}
