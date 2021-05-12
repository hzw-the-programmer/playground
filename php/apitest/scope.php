<?php
$sns = ['20180123000001', '20180123000002', '20180123000003'];
$newSns = [];
//$devices = ['20180123000001' => 1];
//$devices = ['20180123000002' => 2];
$devices = ['20180123000001' => 1, '20180123000003' => 3];
$psn = '';
foreach ($sns as $sn) {
    if (!isset($devices[$sn]) || $newSns) {
        $newSns[] = [$psn, $sn];
    }
    $psn = $sn;
}
echo "$sn\n";
var_dump($newSns);
