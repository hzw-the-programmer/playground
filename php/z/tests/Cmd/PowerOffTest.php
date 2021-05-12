<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
use PHPUnit\Framework\TestCase;
use Hzw\Cmd\PowerOff;

class PowerOffTest extends TestCase
{
    function testPack()
    {
        $cmd = new PowerOff(mktime(0, 0, 0, 1, 23, 2018));

        $this->assertEquals(
            '42',
            bin2hex($cmd->pack())
        );
    }

    function testUnpack()
    {
        $cmd = PowerOff::unpack(hex2bin(
            '42120117000000'
        ));

        $this->assertEquals(
            new PowerOff(mktime(0, 0, 0, 1, 23, 2018)),
            $cmd
        );
    }
}
