<?php
include_once "./db.php";
include_once "../db_config.php";

function test() {
    $db = new Db(DSN, USERNAME, PASSWD);

    $sn = 79173400000005;
    $dt = time();
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

    $db->newChannelsData($sn, $dt, $data);
}

function testGetDevices() {
    
    $db = new Db(DSN, USERNAME, PASSWD);
    $start = microtime(true);
    $devices = $db->getDevices();
    echo (microtime(true) - $start) . "\n";
    var_dump($devices);
}

//test();
testGetDevices();
