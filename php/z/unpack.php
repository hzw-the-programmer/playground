<?php
include_once "./zprotocal.php";

if (count($argv) != 2) {
    echo "Usage: {$argv[0]} data\n";
    exit();
}

$data = hex2bin($argv[1]);

list($valid, $ver, $len, $sn, $id, $dt, $cmd, $data) = z_unpack($data);

echo "valid: " . ($valid ? "TRUE\n" : "FALSE\n");
echo "ver:   ${ver}\n";
echo "len:   ${len}\n";
echo "sn:    ${sn}\n";
echo "id:    ${id}\n";
echo "dt:    " . date("Y-m-d H:i:s", $dt) . "\n";
echo "cmd:   0x" . dechex($cmd) . "\n";
echo "data:  0x" . bin2hex($data) . "\n";

switch ($cmd) {
    case Z_CMD_DATA:
        for ($i = 0; $i < z_get_channels_data_nitem($data); $i++) {
            echo "******\n";
            list($slot, $port, $type, $ad) = z_get_channels_data_item($data, $i);
            echo "slot:  {$slot}\n";
            echo "port:  {$port}\n";
            echo "type:  {$type}\n";
            echo "ad  :  {$ad}\n";
        }
    break;
}
