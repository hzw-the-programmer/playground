<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
include "./zprotocal.php";

function test_pack_unpack_ack() {
    echo "\ntest_pack_unpack_ack\n";

    $sn = 20170123000001;
    $id = 0;
    $dt = mktime(0, 0, 0, 1, 23, 2018);
    $cmd = Z_CMD_ACK;

    $data = z_pack_id(0x12345678);
    $data = z_pack($sn, $id, $dt, $cmd, $data);
    echo bin2hex($data) . "\n";

    list($valid, $ver, $len, $sn, $id, $dt, $cmd, $data) = z_unpack($data);

    echo "valid: " . ($valid ? "TRUE\n" : "FALSE\n");
    echo "ver:   ${ver}\n";
    echo "len:   ${len}\n";
    echo "sn:    ${sn}\n";
    echo "id:    ${id}\n";
    echo "dt:    " . date("Y-m-d H:i:s", $dt) . "\n";
    echo "cmd:   0x" . dechex($cmd) . "\n";
    echo "data:  0x" . bin2hex($data) . "\n";

    echo "******\n";
    echo "ack:   0x" . dechex(z_unpack_id($data)[0]) . "\n";
}

function test_pack_unpack_channels_on() {
    echo "\ntest_pack_unpack_channels_on\n";

    $sn = 20170123000001;
    $id = 1;
    $dt = mktime(0, 0, 0, 1, 23, 2018);
    $cmd = Z_CMD_ON;

    $channels = [];
    for ($i = 1; $i < 3; $i++) {
        for ($j = 0; $j < 2; $j++) {
            $c = new stdClass();
            $c->slot = $i;
            $c->port = $j;
            $channels[] = $c;
        }
    }

    $data = z_pack_channels($channels);
    $data = z_pack($sn, $id, $dt, $cmd, $data);
    echo bin2hex($data) . "\n";

    list($valid, $ver, $len, $sn, $id, $dt, $cmd, $data) = z_unpack($data);

    echo "valid: " . ($valid ? "TRUE\n" : "FALSE\n");
    echo "ver:   ${ver}\n";
    echo "len:   ${len}\n";
    echo "sn:    ${sn}\n";
    echo "id:    ${id}\n";
    echo "dt:    " . date("Y-m-d H:i:s", $dt) . "\n";
    echo "cmd:   0x" . dechex($cmd) . "\n";
    echo "data:  0x" . bin2hex($data) . "\n";

    for ($i = 0; $i < z_get_channels_nitem($data); $i++) {
        echo "******\n";
        list($slot, $port) = z_get_channels_item($data, $i);
        echo "slot:  {$slot}\n";
        echo "port:  {$port}\n";
    }
}

function test_pack_unpack_channels_off() {
    echo "\ntest_pack_unpack_channels_off\n";

    $sn = 20170123000001;
    $id = 1;
    $dt = mktime(0, 0, 0, 1, 23, 2018);
    $cmd = Z_CMD_OFF;

    $channels = [];
    for ($i = 1; $i < 3; $i++) {
        for ($j = 0; $j < 2; $j++) {
            $c = new stdClass();
            $c->slot = $i;
            $c->port = $j;
            $channels[] = $c;
        }
    }

    $data = z_pack_channels($channels);
    $data = z_pack($sn, $id, $dt, $cmd, $data);
    echo bin2hex($data) . "\n";

    list($valid, $ver, $len, $sn, $id, $dt, $cmd, $data) = z_unpack($data);

    echo "valid: " . ($valid ? "TRUE\n" : "FALSE\n");
    echo "ver:   ${ver}\n";
    echo "len:   ${len}\n";
    echo "sn:    ${sn}\n";
    echo "id:    ${id}\n";
    echo "dt:    " . date("Y-m-d H:i:s", $dt) . "\n";
    echo "cmd:   0x" . dechex($cmd) . "\n";
    echo "data:  0x" . bin2hex($data) . "\n";

    for ($i = 0; $i < z_get_channels_nitem($data); $i++) {
        echo "******\n";
        list($slot, $port) = z_get_channels_item($data, $i);
        echo "slot:  {$slot}\n";
        echo "port:  {$port}\n";
    }
}

function test_pack_unpack_channels_data() {
    echo "\ntest_pack_unpack_channels_data\n";

    $sn = 20170123000001;
    $id = 1;
    $dt = mktime(0, 0, 0, 1, 23, 2018);
    $cmd = Z_CMD_DATA;

    $channels = [];
    for ($i = 1; $i < 3; $i++) {
        for ($j = 0; $j < 2; $j++) {
            $c = new stdClass();
            $c->slot = $i;
            $c->port = $j;
            $c->type = 9;
            $c->data = 123.123;
            $channels[] = $c;
        }
    }

    $data = z_pack_channels_data($channels);
    $data = z_pack($sn, $id, $dt, $cmd, $data);
    echo bin2hex($data) . "\n";

    list($valid, $ver, $len, $sn, $id, $dt, $cmd, $data) = z_unpack($data);

    echo "valid: " . ($valid ? "TRUE\n" : "FALSE\n");
    echo "ver:   ${ver}\n";
    echo "len:   ${len}\n";
    echo "sn:    ${sn}\n";
    echo "id:    ${id}\n";
    echo "dt:    " . date("Y-m-d H:i:s", $dt) . "\n";
    echo "cmd:   0x" . dechex($cmd) . "\n";
    echo "data:  0x" . bin2hex($data) . "\n";

    for ($i = 0; $i < z_get_channels_data_nitem($data); $i++) {
        echo "******\n";
        list($slot, $port, $type, $ad) = z_get_channels_data_item($data, $i);
        echo "slot:  {$slot}\n";
        echo "port:  {$port}\n";
        echo "type:  {$type}\n";
        echo "ad  :  {$ad}\n";
    }
}

function test_pack_unpack_channels_status() {
    echo "\ntest_pack_unpack_channels_status\n";

    $sn = 20170123000001;
    $id = 1;
    $dt = mktime(0, 0, 0, 1, 23, 2018);
    $cmd = Z_CMD_STATUS;

    $channels = [];
    for ($i = 1; $i < 3; $i++) {
        for ($j = 0; $j < 2; $j++) {
            $c = new stdClass();
            $c->slot = $i;
            $c->port = $j;
            $c->type = 9;
            $c->status = 8;
            $channels[] = $c;
        }
    }

    $data = z_pack_channels_status($channels);
    $data = z_pack($sn, $id, $dt, $cmd, $data);
    echo bin2hex($data) . "\n";

    list($valid, $ver, $len, $sn, $id, $dt, $cmd, $data) = z_unpack($data);

    echo "valid: " . ($valid ? "TRUE\n" : "FALSE\n");
    echo "ver:   ${ver}\n";
    echo "len:   ${len}\n";
    echo "sn:    ${sn}\n";
    echo "id:    ${id}\n";
    echo "dt:    " . date("Y-m-d H:i:s", $dt) . "\n";
    echo "cmd:   0x" . dechex($cmd) . "\n";
    echo "data:  0x" . bin2hex($data) . "\n";

    for ($i = 0; $i < z_get_channels_status_nitem($data); $i++) {
        echo "******\n";
        list($slot, $port, $type, $status) = z_get_channels_status_item($data, $i);
        echo "slot    : {$slot}\n";
        echo "port    : {$port}\n";
        echo "type    : {$type}\n";
        echo "status  : {$status}\n";
    }
}

//test_pack_unpack_ack();
//test_pack_unpack_channels_on();
//test_pack_unpack_channels_off();
//test_pack_unpack_channels_data();
test_pack_unpack_channels_status();
