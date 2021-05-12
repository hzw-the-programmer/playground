<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
include "./zprotocal.php";

const HEART_BEAT_INTERVAL = 10000;
const DATA_INTERVAL = 20000;
const STATUS_INTERVAL = 30000;

if (count($argv) != 3) {
    echo "Usage: {$argv[0]} sn ip\n";
    exit();
}
$sn = $argv[1];
$ip = $argv[2];
$id = 0;
$client = new swoole_client(SWOOLE_SOCK_UDP, SWOOLE_SOCK_ASYNC);

$client->on("Connect", function($client) {
    echo "\nonConnect: pid=" . getmypid() . "\n";

    swoole_timer_tick(HEART_BEAT_INTERVAL, function($timerid) {
        echo "\nheartBeatTimer: pid=" . getmypid() . "\n";
        global $sn, $id, $client;

        $id++;
        $dt = time();
        $cmd = Z_CMD_HEART_BEAT;
        $data = z_pack($sn, $id, $dt, $cmd);
        $client->send($data);

        echo bin2hex($data) . "\n";
    });

    swoole_timer_tick(DATA_INTERVAL, function($timerid) {
        echo "\ndataTimer: pid=" . getmypid() . "\n";
        global $sn, $id, $client;

        $id++;
        $dt = time();
        $cmd = Z_CMD_DATA;

        $channels = [];
        for ($i = 1; $i < 2; $i++) {
            for ($j = 0; $j < 4; $j++) {
                $c = new stdClass();
                $c->slot = $i;
                $c->port = $j;
                $c->type = 8;
                $c->data = 123.123;
                $channels[] = $c;
            }
        }

        $data = z_pack_channels_data($channels);
        $data = z_pack($sn, $id, $dt, $cmd, $data);
        $client->send($data);

        echo bin2hex($data) . "\n";
    });

    swoole_timer_tick(STATUS_INTERVAL, function($timerid) {
        echo "\nstatusTimer: pid=" . getmypid() . "\n";
        global $sn, $id, $client;

        $id++;
        $dt = time();
        $cmd = Z_CMD_STATUS;

        $channels = [];
        for ($i = 1; $i < 2; $i++) {
            for ($j = 0; $j < 4; $j++) {
                $c = new stdClass();
                $c->slot = $i;
                $c->port = $j;
                $c->type = 8;
                $c->status = 8;
                $channels[] = $c;
            }
        }

        $data = z_pack_channels_status($channels);
        $data = z_pack($sn, $id, $dt, $cmd, $data);
        $client->send($data);

        echo bin2hex($data) . "\n";
    });
});

$client->on("Receive", function($client, $data) {
    echo "\nonReceive: pid=" . getmypid() . "\n";

    echo bin2hex($data) . "\n";
});

$client->connect($argv[2], 19268);

echo "\npid=" . getmypid() . "\n";
