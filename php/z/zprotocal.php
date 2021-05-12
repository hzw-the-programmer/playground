<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
const Z_VER = "P3";

const Z_VER_INDEX = 0;
const Z_VER_LEN = 2;

const Z_LEN_INDEX = 2;
const Z_LEN_LEN = 2;

const Z_SN_INDEX = 4;
const Z_SN_LEN = 8;

const Z_CHKSUM_BEGIN_INDEX = Z_SN_INDEX;

const Z_ID_INDEX = 12;
const Z_ID_LEN = 4;

const Z_DT_INDEX = 16;
const Z_DT_LEN = 6;

const Z_CMD_INDEX = 22;
const Z_CMD_LEN = 1;

const Z_CHANNELS_ITEM_LEN = 1;
const Z_CHANNELS_DATA_ITEM_LEN = 7;
const Z_CHANNELS_STATUS_ITEM_LEN = 3;

const Z_CMD_ACK = 0x24;
const Z_CMD_DATA = 0x41;
const Z_CMD_OFF = 0x50;
const Z_CMD_ON = 0x51;
const Z_CMD_STATUS = 0x51;
const Z_CMD_52 = 0x52;
const Z_CMD_HEART_BEAT = 0xFE;

function z_unpack_ver($data) {
    return [substr($data, 0, Z_VER_LEN), Z_VER_LEN];
}

function z_unpack_len($data) {
    return [unpack("n", $data)[1], Z_LEN_LEN];
}

function z_unpack_sn($data) {
    return [unpack("P", $data)[1], Z_SN_LEN];
}

function z_unpack_id($data) {
    return [unpack("N", $data)[1], Z_ID_LEN];
}

function z_unpack_dt($data) {
    $year = ord($data[0]) + 2000;
    $mon = ord($data[1]);
    $day = ord($data[2]);
    $hour = ord($data[3]);
    $min = ord($data[4]);
    $sec = ord($data[5]);

    return [mktime($hour, $min, $sec, $mon, $day, $year), Z_DT_LEN];
}

function z_unpack_cmd($data) {
    return [ord($data[0]), 1];
}

function z_chksum($data) {
    $chksum = 0;
    for ($i = 0; $i < strlen($data); $i++) {
        $chksum ^= ord($data[$i]);
    }
    return $chksum;
}

function z_valid($data) {
    if (strlen($data) < Z_VER_LEN + Z_LEN_LEN) {
        return FALSE;
    }

    $r = z_unpack_ver($data);
    if (strcmp(Z_VER, $r[0]) != 0) {
        return FALSE;
    }

    $data = substr($data, $r[1]);
    $r = z_unpack_len($data);

    $data = substr($data, $r[1]);
    if (strlen($data) != $r[0]) {
        return FALSE;
    }

    if (ord($data[$r[0] - 1]) != z_chksum(substr($data, 0, $r[0] - 1))) {
        return FALSE;
    }

    return TRUE;
}

function z_unpack($data) {
    list($valid, $ver, $len, $sn, $id, $dt, $cmd) = [
        FALSE, "", 0, 0, 0, 0, 0
    ];

    $valid = z_valid($data);
    if (!$valid) goto end;

    $r = z_unpack_ver($data);
    $ver = $r[0];

    $data = substr($data, $r[1]);
    $r = z_unpack_len($data);
    $len = $r[0];

    $data = substr($data, $r[1]);
    $r = z_unpack_sn($data);
    $sn = $r[0];

    $data = substr($data, $r[1]);
    $r = z_unpack_id($data);
    $id = $r[0];

    $data = substr($data, $r[1]);
    $r = z_unpack_dt($data);
    $dt = $r[0];

    $data = substr($data, $r[1]);
    $r = z_unpack_cmd($data);
    $cmd = $r[0];

    $data = substr($data, $r[1]);

    end:
        return [$valid, $ver, $len, $sn, $id, $dt, $cmd, $data];
}

function z_pack_sn($sn) {
    return pack("P", $sn);
}

function z_pack_id($id) {
    return pack("N", $id);
}

function z_pack_dt($dt) {
    $year = chr(date("Y", $dt) - 2000);
    $mon = chr(date("m", $dt));
    $day = chr(date("d", $dt));
    $hour = chr(date("H", $dt));
    $min = chr(date("i", $dt));
    $sec = chr(date("s", $dt));

    return $year . $mon . $day . $hour . $min . $sec;
}

function z_pack_cmd($cmd) {
    return chr($cmd);
}

function z_pack_len($len) {
    return pack("n", $len);
}

function z_pack($sn, $id, $dt, $cmd, $data = "") {
    $pkt = z_pack_sn($sn);
    $pkt .= z_pack_id($id);
    $pkt .= z_pack_dt($dt);
    $pkt .= z_pack_cmd($cmd);
    $pkt .= $data;
    $pkt .= chr(z_chksum($pkt));
    $pkt = Z_VER . z_pack_len(strlen($pkt)) . $pkt;

    return $pkt;
}

function z_pack_channel($channel) {
    return pack("C", $channel->slot << 4 | $channel->port);
}

function z_pack_channels($channels) {
    $data = pack("C", count($channels));
    foreach ($channels as $channel) {
        $data .= z_pack_channel($channel);
    }

    return $data;
}

function z_get_channels_nitem($data) {
    return unpack("C", $data)[1];
}

function z_get_channels_item($data, $i) {
    $data = substr($data, 1 + $i * Z_CHANNELS_ITEM_LEN);
    $data = unpack("C", $data)[1];

    $slot = $data >> 4;
    $port = $data & 0x0F;

    return [$slot, $port];
}

function z_pack_channel_data($channel) {
    $data = pack("C", $channel->slot);
    $data .= pack("C", $channel->port);
    $data .= pack("C", $channel->type);
    $data .= pack("g", $channel->data);

    return $data;
}

function z_pack_channels_data($channels) {
    $data = pack("C", count($channels));

    foreach ($channels as $channel) {
        $data .= z_pack_channel_data($channel);
    }

    return $data;
}

function z_get_channels_data_nitem($data) {
    return ord($data[0]);
}

function z_get_channels_data_item($data, $i) {
    $data = substr($data, 1 + $i * Z_CHANNELS_DATA_ITEM_LEN);
    $slot = ord($data[0]);
    $port = ord($data[1]);
    $type = ord($data[2]);
    $data = substr($data, 3);
    $ad = unpack("g", $data)[1];

    return [$slot, $port, $type, $ad];
}

function z_pack_channel_status($channel) {
    $data = pack("C", ($channel->slot << 4) | ($channel->port));
    $data .= pack("C", $channel->type);
    $data .= pack("C", $channel->status);

    return $data;
}

function z_pack_channels_status($channels) {
    $data = pack("C", count($channels));

    foreach ($channels as $channel) {
        $data .= z_pack_channel_status($channel);
    }

    return $data;
}

function z_get_channels_status_nitem($data) {
    return unpack("C", $data[0])[1];
}

function z_get_channels_status_item($data, $i) {
    $data = substr($data, 1 + $i * Z_CHANNELS_STATUS_ITEM_LEN);
    $sp = unpack("C", $data[0])[1];
    $slot = $sp >> 4;
    $port = $sp & 0x0F;
    $type = unpack("C", $data[1])[1];
    $status = unpack("C", $data[2])[1];

    return [$slot, $port, $type, $status];
}
