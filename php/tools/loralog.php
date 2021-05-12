<?php

$prog = array_shift($argv);
if (count($argv) == 0) {
    die("Usage: $prog file [sn...]\n");
}
$file = array_shift($argv);
$sns = [];
foreach ($argv as $arg) {
    $sns[] = intval($arg);
}

echo "sn, time, gateway, appid, devaddr, fcnt, remark, freq, stat, lsnr, rssi, payload\n";

$f = fopen($file, 'r');

while ($l = fgets($f)) {
    if (preg_match("/Recv: (.*), (.*), (.*)/", $l, $matches)) {
        $t = $matches[1];
        $gw = $matches[2];
        $json = json_decode($matches[3]);
        if (!isset($json->rxpk)) continue;
        foreach ($json->rxpk as $rxpk) {
            $bin = base64_decode($rxpk->data);

            if (strlen($bin) < 6) continue;

            $i0 = ord($bin[0]);
            $i1 = ord($bin[1]);
            $i2 = ord($bin[2]);
            $i3 = ord($bin[3]);
            $i4 = ord($bin[4]);
            $i5 = ord($bin[5]);

            if ($i0 != 0x00 && $i0 != 0x40 && $i0 != 0x80) continue;

            $remark = '-1';
            $pb = 6;
            if ($i0 == 0x00) {
                if ($i3&0x10) $remark = 'request';
                else $remark = 'join';
            } else if ($i0 == 0x40) {
                if (($i3&0x0f)==0x03) {
                    if (strlen($bin) < 9) continue;
                    $i6 = ord($bin[6]);
                    $i7 = ord($bin[7]);
                    $i8 = ord($bin[8]);
                    $pb = 9;
                    if ($i6 == 0x01) {
                        $remark = $i7 | ($i8<<8);
                    }
                }
            }

            $appid = ($i1&0x0c)>>2;
            $devaddr = ( ( ( ($i1>>4)<<2 ) | ($i1&0x03) )<<8 ) | $i2;
            $fcnt = $i4 | ($i5<<8);

            $bin = substr($bin, $pb);

            if (strlen($bin) < 12) {
                $sn = 0;
            } else {
                $sn = unpack('C2h/nlen/Jsn', $bin)['sn'];
            }
            if ($sns && array_search($sn, $sns) === false) continue;
            $sn = sprintf('%014u', $sn);
            
            echo sprintf(
                "%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s\n",
                $sn, $t, $gw, $appid, $devaddr, $fcnt, $remark,
                $rxpk->freq, $rxpk->stat, $rxpk->lsnr, $rxpk->rssi, bin2hex($bin)
            );
        }
    }
}
