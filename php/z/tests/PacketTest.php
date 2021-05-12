<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
use PHPUnit\Framework\TestCase;
use Hzw\Utils;
use Hzw\Channel;
use Hzw\Param;
use Hzw\Header;
use Hzw\Cmd\Ack;
use Hzw\Cmd\HeartBeat;
use Hzw\Cmd\Data;
use Hzw\Cmd\Status;
use Hzw\Cmd\GetSn;
use Hzw\Cmd\SetSn;
use Hzw\Cmd\GetParam;
use Hzw\Cmd\SetParam;
use Hzw\Cmd\Open;
use Hzw\Cmd\Close;
use Hzw\Packet;

class PacketTest extends TestCase {
    function setUp() {
        Packet::init();
    }

    function tearDown() {
        Packet::deinit();
    }

    function testPack() {
        $sns = ['20170123000001', '00170123000002'];
        $seq = 0x12345678;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt);
        
        $body = new Body("\x23");
        
        $packet = new Packet($header, $body);
        
        $this->assertEquals(
            '5334' .
            '001d' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '2e000000279c1ef8c2' .
            '23' .
            '07',
            bin2hex($packet->pack()));
    }

    function testPackSetSn() {
        $sns = [];
        $seq = 0x12345678;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt);
        
        $body = new SetSn('20170123000001');
        //$body = new SetSn(['20170123000001', '20170123000002']);
        
        $packet = new Packet($header, $body);
        
        $this->assertEquals(
            '5334' .
            '001c' .
            '0000000000000000' .
            '12345678' .
            '120117170b0b' .
            '05' .
            '00001258390438c1' .
            '90',
            bin2hex($packet->pack()));
    }

    function testPackAckSetSn() {
        $sns = ['20170123000001'];
        $seq = 0x23232323;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt, 'T');
        
        $body = new Ack(0x12345678, 0x00, SetSn::CMD, null);
        
        $packet = new Packet($header, $body);
        
        $this->assertEquals(
            '5434' .
            '001a' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '24' .
            '12345678' .
            '00' .
            '05' .
            'b4',
            bin2hex($packet->pack()));
    }

    function testPackGetSn() {
        $sns = [];
        $seq = 0x12345678;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt);
        
        $body = new GetSn;
        
        $packet = new Packet($header, $body);
        
        $this->assertEquals(
            '5334' .
            '0014' .
            '0000000000000000' .
            '12345678' .
            '120117170b0b' .
            '06' .
            '1d',
            bin2hex($packet->pack()));
    }

    function testPackAckGetSn() {
        $sns = ['20170123000001'];
        $seq = 0x23232323;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt, 'T');
        
        $body = new Ack(0x12345678, 0x00, GetSn::CMD, ['20170123000001', '00170123000002']);
        
        $packet = new Packet($header, $body);
        
        $this->assertEquals(
            '5434' .
            '002b' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '24' .
            '12345678' .
            '00' .
            '06' .
            '02' .
            '00001258390438c1' .
            '000000279c1ef8c2' .
            'a4',
            bin2hex($packet->pack()));
    }

    function testPackOpen() {
        $sns = ['20170123000001', '00170123000002'];
        $seq = 0x12345678;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt);
        
        $body = new Open($this->getChannels());
        
        $packet = new Packet($header, $body);
        
        $this->assertEquals(
            '5334' .
            '002c' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '2e' .
            '000000279c1ef8c2' .
            '49' .
            '0e' .
            '1011121320212223303132334041' .
            '62',
            bin2hex($packet->pack()));
    }

    function testPackAckOpen() {
        $sns = ['20170123000001', '00170123000002'];
        $seq = 0x23232323;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt, 'T');

        $body = new Ack(0x12345678, 0x00, Open::CMD, null);

        $packet = new Packet($header, $body);

        $this->assertEquals(
            '5434' .
            '0023' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '2e' .
            '000000279c1ef8c2' .
            '24' .
            '12345678' .
            '00' .
            '49' .
            '49',
            bin2hex($packet->pack()));
    }

    function testPackClose() {
        $sns = ['20170123000001'];
        $seq = 0x12345678;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt);
        
        $body = new Close($this->getChannels());
        
        $packet = new Packet($header, $body);
        
        $this->assertEquals(
            '5334' .
            '0023' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '50' .
            '0e' .
            '1011121320212223303132334041' .
            'ca',
            bin2hex($packet->pack()));
    }

    function testPackAckClose() {
        $sns = ['20170123000001'];
        $seq = 0x23232323;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt, 'T');

        $body = new Ack(0x12345678, 0x00, Close::CMD, null);

        $packet = new Packet($header, $body);

        $this->assertEquals(
            '5434' .
            '001a' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '24' .
            '12345678' .
            '00' .
            '50' .
            'e1',
            bin2hex($packet->pack()));
    }

    function testPackStatus()
    {
        $sns = ['20170123000001', '00170123000002'];
        $seq = 0x12345678;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt, 'T');
        
        $body = new Status($this->getChannels());
        
        $packet = new Packet($header, $body);
        
        $this->assertEquals(
            '5434' .
            '0048' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '2e' .
            '000000279c1ef8c2' .
            '51' .
            '0e' .
            '100820110820120800130800' .
            '200c20210c20220c00230c00' .
            '300c20310c20320c00330c00' .
            '400920410900' .
            '5a',
            bin2hex($packet->pack()));
    }

    function testPackAckStatus() {
        $sns = ['20170123000001', '00170123000002'];
        $seq = 0x23232323;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt);

        $body = new Ack(0x12345678, 0x00, Status::CMD, null);

        $packet = new Packet($header, $body);

        $this->assertEquals(
            '5334' .
            '0023' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '2e' .
            '000000279c1ef8c2' .
            '24' .
            '12345678' .
            '00' .
            '51' .
            '51',
            bin2hex($packet->pack()));
    }

    function testPackData()
    {
        $sns = ['20170123000001'];
        $seq = 0x12345678;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt, 'T');
        
        $body = new Data($this->getChannels());
        
        $packet = new Packet($header, $body);
        
        $this->assertEquals(
            '5434' .
            '0077' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '41' .
            '0e' .
            '01000856ffca3a01010856ffca3a010208909e303a010308909e303a' .
            '02000ce8d1054202010ce8d1054202020c389b594102030c389b5941' .
            '03000ce8d1054203010ce8d1054203020c389b594103030c389b5941' .
            '04000902241a3904010903366739' .
            'b5',
            bin2hex($packet->pack()));
    }

    function testPackAckData() {
        $sns = ['20170123000001'];
        $seq = 0x23232323;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt);

        $body = new Ack(0x12345678, 0x00, Data::CMD, null);

        $packet = new Packet($header, $body);

        $this->assertEquals(
            '5334' .
            '001a' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '24' .
            '12345678' .
            '00' .
            '41' .
            'f0',
            bin2hex($packet->pack()));
    }

    function testPackGetParam() {
        $sns = ['20170123000001'];
        $seq = 0x12345678;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt);
        
        $body = new GetParam(Channel::WS);
        
        $packet = new Packet($header, $body);
        
        $this->assertEquals(
            '5334' .
            '0015' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '52' .
            '09' .
            'ce',
            bin2hex($packet->pack()));
    }

    function testPackAckGetParam() {
        $sns = ['20170123000001'];
        $seq = 0x23232323;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt, 'T');

        $body = new Ack(0x12345678, 0x00, GetParam::CMD, new Param(Channel::WS, 600, 1, 35, 1, 600, 1200, 1800));

        $packet = new Packet($header, $body);

        $this->assertEquals(
            '5434' .
            '0029' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '24' .
            '12345678' .
            '00' .
            '52' .
            '095802df00720701005802b0040807' .
            'fa',
            bin2hex($packet->pack()));
    }

    function testPackSetParam() {
        $sns = ['20170123000001', '00170123000002'];
        $seq = 0x12345678;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt);
        
        $body = new SetParam(new Param(Channel::WS, 600, 1, 35, 1, 600, 1200, 1800));
        
        $packet = new Packet($header, $body);
        
        $this->assertEquals(
            '5334' .
            '002c' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '2e' .
            '000000279c1ef8c2' .
            '53' .
            '095802df00720701005802b0040807' .
            '6e',
            bin2hex($packet->pack()));
    }

    function testPackAckSetParam() {
        $sns = ['20170123000001', '00170123000002'];
        $seq = 0x23232323;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt, 'T');

        $body = new Ack(0x12345678, 0x00, SetParam::CMD, null);

        $packet = new Packet($header, $body);

        $this->assertEquals(
            '5434' .
            '0023' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '2e' .
            '000000279c1ef8c2' .
            '24' .
            '12345678' .
            '00' .
            '53' .
            '53',
            bin2hex($packet->pack()));
    }

    function testPackHeartBeat() {
        $sns = ['20170123000001', '00170123000002'];
        $seq = 0x12345678;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt, 'T');
        
        $body = new HeartBeat(0x96);
        
        $packet = new Packet($header, $body);
        
        $this->assertEquals(
            '5434' .
            '001e' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '2e' .
            '000000279c1ef8c2' .
            'fe' .
            '96' .
            '4c',
            bin2hex($packet->pack()));
    }

    function testPackAckHeartBeat() {
        $sns = ['20170123000001', '00170123000002'];
        $seq = 0x23232323;
        $dt = mktime(23, 11, 11, 1, 23, 2018);
        $header = new Header($sns, $seq, $dt);
        
        $body = new Ack(0x12345678, 0x00, HeartBeat::CMD, null);
        
        $packet = new Packet($header, $body);
        
        $this->assertEquals(
            '5334' .
            '0023' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '2e' .
            '000000279c1ef8c2' .
            '24' .
            '12345678' .
            '00' .
            'fe' .
            'fe',
            bin2hex($packet->pack()));
    }

    function testUnpackTooShort() {
        $packet = Packet::unpack(hex2bin(
            '5334' .
            '00'
        ));
        
        $this->assertEquals(new Packet(null, null, Packet::TOO_SHORT), $packet);
    }

    function testUnpackWrongLen() {
        $packet = Packet::unpack(hex2bin(
            '5334' .
            '001d'
        ));
        
        $this->assertEquals(new Packet(null, null, Packet::WRONG_LEN), $packet);
    }

    function testUnpackWrongChksum() {
        $packet = Packet::unpack(hex2bin(
            '5334' .
            '001d' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '2e000000279c1ef8c2' .
            '23' .
            '37'
        ));
        
        $this->assertEquals(new Packet(null, null, Packet::WRONG_CHKSUM), $packet);
    }

    function testUnpackNoBodyUnpacker() {
        $packet = Packet::unpack(hex2bin(
            '5334' .
            '001d' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '2e000000279c1ef8c2' .
            '23' .
            '07'
        ));

        $this->assertEquals(new Packet(null, null, Packet::NO_BODY_UNPACKER), $packet);
    }

    function testUnpack() {
        Packet::registerBodyUnpacker(Body::class);
        $packet = Packet::unpack(hex2bin(
            '5334' .
            '001d' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '2e000000279c1ef8c2' .
            '23' .
            '07'
        ));
        Packet::unregisterBodyUnpacker(Body::class);

        $header = $packet->getHeader();

        $this->assertEquals('S', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(29, $header->getLen());
        $this->assertEquals(['20170123000001', '00170123000002'], $header->getSns());
        $this->assertEquals(0x12345678, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = new Body("\x23");

        $this->assertEquals($body, $packet->getBody());
    }

    function testUnpackSetSn() {
        $packet = Packet::unpack(hex2bin(
            '5334' .
            '001c' .
            '0000000000000000' .
            '12345678' .
            '120117170b0b' .
            '05' .
            '00001258390438c1' .
            '90'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('S', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(28, $header->getLen());
        $this->assertEquals(['0000000000000000'], $header->getSns());
        $this->assertEquals(0x12345678, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = new SetSn('20170123000001');

        $this->assertEquals($body, $packet->getBody());
    }

    function testUnpackAckSetSn() {
        $packet = Packet::unpack(hex2bin(
            '5434' .
            '001a' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '24' .
            '12345678' .
            '00' .
            '05' .
            'b4'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('T', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(26, $header->getLen());
        $this->assertEquals(['20170123000001'], $header->getSns());
        $this->assertEquals(0x23232323, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = new Ack(0x12345678, 0x00, SetSn::CMD, null);

        $this->assertEquals($body, $packet->getBody());
    }

    function testUnpackGetSn() {
        $packet = Packet::unpack(hex2bin(
            '5334' .
            '0014' .
            '0000000000000000' .
            '12345678' .
            '120117170b0b' .
            '06' .
            '1d'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('S', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(20, $header->getLen());
        $this->assertEquals(['0000000000000000'], $header->getSns());
        $this->assertEquals(0x12345678, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = new GetSn();

        $this->assertEquals($body, $packet->getBody());
    }

    function testUnpackAckGetSn() {
        $packet = Packet::unpack(hex2bin(
            '5434' .
            '002b' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '24' .
            '12345678' .
            '00' .
            '06' .
            '02' .
            '00001258390438c1' .
            '000000279c1ef8c2' .
            'a4'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('T', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(43, $header->getLen());
        $this->assertEquals(['20170123000001'], $header->getSns());
        $this->assertEquals(0x23232323, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = new Ack(0x12345678, 0x00, GetSn::CMD, ['20170123000001', '00170123000002']);

        $this->assertEquals($body, $packet->getBody());
    }

    function testUnpackOpen()
    {
        $packet = Packet::unpack(hex2bin(
            '5334' .
            '002c' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '2e' .
            '000000279c1ef8c2' .
            '49' .
            '0e' .
            '1011121320212223303132334041' .
            '62'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('S', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(44, $header->getLen());
        $this->assertEquals(['20170123000001', '00170123000002'], $header->getSns());
        $this->assertEquals(0x12345678, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = $packet->getBody();

        $channels = $this->getChannels();
        foreach ($channels as $channel) {
            $channel->type = 0;
            $channel->status = null;
            $channel->data = null;
        }
        $_body = new Open($channels);

        $this->assertEquals($_body, $body);
    }

    function testUnpackAckOpen() {
        $packet = Packet::unpack(hex2bin(
            '5434' .
            '0023' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '2e' .
            '000000279c1ef8c2' .
            '24' .
            '12345678' .
            '00' .
            '49' .
            '49'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('T', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(35, $header->getLen());
        $this->assertEquals(['20170123000001', '00170123000002'], $header->getSns());
        $this->assertEquals(0x23232323, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = new Ack(0x12345678, 0x00, Open::CMD, null);

        $this->assertEquals($body, $packet->getBody());
    }

    function testUnpackClose()
    {
        $packet = Packet::unpack(hex2bin(
            '5334' .
            '0023' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '50' .
            '0e' .
            '1011121320212223303132334041' .
            'ca'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('S', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(35, $header->getLen());
        $this->assertEquals(['20170123000001'], $header->getSns());
        $this->assertEquals(0x12345678, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = $packet->getBody();

        $channels = $this->getChannels();
        foreach ($channels as $channel) {
            $channel->type = 0;
            $channel->status = null;
            $channel->data = null;
        }
        $_body = new Close($channels);

        $this->assertEquals($_body, $body);
    }

    function testUnpackAckClose() {
        $packet = Packet::unpack(hex2bin(
            '5434' .
            '001a' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '24' .
            '12345678' .
            '00' .
            '50' .
            'e1'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('T', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(26, $header->getLen());
        $this->assertEquals(['20170123000001'], $header->getSns());
        $this->assertEquals(0x23232323, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = new Ack(0x12345678, 0x00, Close::CMD, null);

        $this->assertEquals($body, $packet->getBody());
    }

    function testUnpackStatus()
    {
        $packet = Packet::unpack(hex2bin(
            '5434' .
            '0048' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '2e' .
            '000000279c1ef8c2' .
            '51' .
            '0e' .
            '100820110820120800130800' .
            '200c20210c20220c00230c00' .
            '300c20310c20320c00330c00' .
            '400920410900' .
            '5a'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('T', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(72, $header->getLen());
        $this->assertEquals(['20170123000001', '00170123000002'], $header->getSns());
        $this->assertEquals(0x12345678, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = $packet->getBody();

        $channels = $this->getChannels();
        foreach ($channels as $channel) {
            $channel->data = null;
        }
        $_body = new Status($channels);

        $this->assertEquals($_body, $body);
    }

    function testUnpackAckStatus() {
        $packet = Packet::unpack(hex2bin(
            '5334' .
            '0023' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '2e' .
            '000000279c1ef8c2' .
            '24' .
            '12345678' .
            '00' .
            '51' .
            '51'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('S', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(35, $header->getLen());
        $this->assertEquals(['20170123000001', '00170123000002'], $header->getSns());
        $this->assertEquals(0x23232323, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = new Ack(0x12345678, 0x00, Status::CMD, null);

        $this->assertEquals($body, $packet->getBody());
    }

    function testUnpackData()
    {
        $packet = Packet::unpack(hex2bin(
            '5434' .
            '0077' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '41' .
            '0e' .
            '01000856ffca3a01010856ffca3a010208909e303a010308909e303a' .
            '02000ce8d1054202010ce8d1054202020c389b594102030c389b5941' .
            '03000ce8d1054203010ce8d1054203020c389b594103030c389b5941' .
            '04000902241a3904010903366739' .
            'b5'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('T', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(119, $header->getLen());
        $this->assertEquals(['20170123000001'], $header->getSns());
        $this->assertEquals(0x12345678, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = $packet->getBody();
        
        foreach ($body->getChannels() as $channel) {
            $channel->data = round($channel->data, 1);
        }

        $channels = $this->getChannels();
        foreach ($channels as $channel) {
            $channel->status = null;
        }
        $_body = new Data($channels);

        $this->assertEquals($_body, $body);
    }

    function testUnpackAckData() {
        $packet = Packet::unpack(hex2bin(
            '5334' .
            '001a' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '24' .
            '12345678' .
            '00' .
            '41' .
            'f0'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('S', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(26, $header->getLen());
        $this->assertEquals(['20170123000001'], $header->getSns());
        $this->assertEquals(0x23232323, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = new Ack(0x12345678, 0x00, Data::CMD, null);

        $this->assertEquals($body, $packet->getBody());
    }

    function testUnpackGetParam() {
        $packet = Packet::unpack(hex2bin(
            '5334' .
            '0015' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '52' .
            '09' .
            'ce'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('S', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(21, $header->getLen());
        $this->assertEquals(['20170123000001'], $header->getSns());
        $this->assertEquals(0x12345678, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = new GetParam(Channel::WS);

        $this->assertEquals($body, $packet->getBody());
    }

    function testUnpackAckGetParam() {
        $packet = Packet::unpack(hex2bin(
            '5434' .
            '0029' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '24' .
            '12345678' .
            '00' .
            '52' .
            '095802df00720701005802b0040807' .
            'fa'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('T', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(41, $header->getLen());
        $this->assertEquals(['20170123000001'], $header->getSns());
        $this->assertEquals(0x23232323, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = $packet->getBody();
        $param = $body->getData();
        $param->lowerLimit = round($param->lowerLimit, 0);
        $param->upperLimit = round($param->upperLimit, 0);

        $param = new Param(Channel::WS, 600, 1, 35, 1, 600, 1200, 1800);
        $_body = new Ack(0x12345678, 0x00, GetParam::CMD, $param);

        $this->assertEquals($_body, $body);
    }

    function testUnpackSetParam() {
        $packet = Packet::unpack(hex2bin(
            '5334' .
            '002c' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '2e' .
            '000000279c1ef8c2' .
            '53' .
            '095802df00720701005802b0040807' .
            '6e'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('S', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(44, $header->getLen());
        $this->assertEquals(['20170123000001', '00170123000002'], $header->getSns());
        $this->assertEquals(0x12345678, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = $packet->getBody();
        $param = $body->getParam();
        $param->lowerLimit = round($param->lowerLimit, 0);
        $param->upperLimit = round($param->upperLimit, 0);

        $param = new Param(Channel::WS, 600, 1, 35, 1, 600, 1200, 1800);
        $_body = new SetParam($param);

        $this->assertEquals($_body, $body);
    }

    function testUnpackAckSetParam() {
        $packet = Packet::unpack(hex2bin(
            '5434' .
            '0023' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '2e' .
            '000000279c1ef8c2' .
            '24' .
            '12345678' .
            '00' .
            '53' .
            '53'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('T', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(35, $header->getLen());
        $this->assertEquals(['20170123000001', '00170123000002'], $header->getSns());
        $this->assertEquals(0x23232323, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = new Ack(0x12345678, 0x00, SetParam::CMD, null);

        $this->assertEquals($body, $packet->getBody());
    }

    function testUnpackHeartBeat() {
        $packet = Packet::unpack(hex2bin(
            '5434' .
            '001e' .
            '00001258390438c1' .
            '12345678' .
            '120117170b0b' .
            '2e' .
            '000000279c1ef8c2' .
            'fe' .
            '96' .
            '4c'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('T', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(30, $header->getLen());
        $this->assertEquals(['20170123000001', '00170123000002'], $header->getSns());
        $this->assertEquals(0x12345678, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = new HeartBeat(0x96);

        $this->assertEquals($body, $packet->getBody());
    }

    function testUnpackAckHeartBeat() {
        $packet = Packet::unpack(hex2bin(
            '5334' .
            '0023' .
            '00001258390438c1' .
            '23232323' .
            '120117170b0b' .
            '2e' .
            '000000279c1ef8c2' .
            '24' .
            '12345678' .
            '00' .
            'fe' .
            'fe'
        ));

        $header = $packet->getHeader();

        $this->assertEquals('S', $header->getFrom());
        $this->assertEquals('4', $header->getVer());
        $this->assertEquals(35, $header->getLen());
        $this->assertEquals(['20170123000001', '00170123000002'], $header->getSns());
        $this->assertEquals(0x23232323, $header->getSeq());
        $this->assertEquals(mktime(23, 11, 11, 1, 23, 2018), $header->getDt());

        $body = new Ack(0x12345678, 0x00, HeartBeat::CMD, null);

        $this->assertEquals($body, $packet->getBody());
    }

    function getChannels($maxSlot = 4, $maxPort = 4) {
        $channels = [];

        for ($slot = 1; $slot < $maxSlot + 1; $slot++) {
            for ($port = 0; $port < $maxPort; $port++) {
                $type = Channel::INVALID;
                $data = 0.0;
                $status = 0x00;

                if ($slot < 2) {
                    $type = Channel::GND_H;
                    $data = $port < 2 ? 17.7 : 7.7;
                    $status = $port < 2 ? 0x20 : 0x00;
                } else if ($slot < 4) {
                    $type = Channel::GND_L;
                    $data = $port < 2 ? 5.5 : 2.2;
                    $status = $port < 2 ? 0x20 : 0x00;
                } else if ($port < 2) {
                    $type = Channel::WS;
                    $data = $port < 1 ? 0.6 : 0.9;
                    $status = $port < 1 ? 0x20 : 0x00;
                }

                if (Channel::INVALID !== $type) {
                    $channel = new Channel($slot, $port, $type);
                    $channel->data = $data;
                    $channel->status = $status;
                    $channels[] = $channel;
                }
            }
        }

        return $channels;
    }
}

class Body {
    function __construct($bin) {
        $this->bin = $bin;
    }

    function pack() {
        return $this->bin;
    }

    static function unpack($bin) {
        return new Body($bin);
    }
}
