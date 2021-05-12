<?php
if ($argc !== 6) {
    die("{$argv[0]} host port database user pwd\n");
}

$host = $argv[1];
$port = $argv[2];
$database = $argv[3];
$user = $argv[4];
$pwd = $argv[5];

$dsn = "sqlsrv:Server=$host,$port;Database=$database";


try {
    $dt = time();
    $sum = 0;
    for ($count = 0; $count < 100; $count++) {
        $st = microtime(true);
        $dbh = new PDO($dsn, $user, $pwd);
        $sql = "EXEC deviceSignal '20180123000001', -88"; // 0.014989650249481
        //$sql = "EXEC new_channel_realtime_status '20180123000001', 1, 0, 8, $dt, 0x01, $dt, 0x20"; // 0.027133727073669
        //$sql = "EXEC new_channel_status '20180123000001', 1, 0, 8, $dt, 0x01, $dt, 0x20"; // 0.017730433940887
        //$sql = "EXEC new_channel_status '20180123000001', 1, 0, 8, $dt, 0x01, $dt, 0x20"; // 0.022602977752686 updateOfflineTime
        //$sql = "EXEC new_device_realtime_status '20180123000001', $dt, 259"; // 256 // 0.088965306282043
        //$sql = "EXEC new_channel_data '20180123000001', 1, 0, 8, $dt, 0x01, $dt, 3.3"; // 0.021416311264038
        $dbh->query($sql);
        $sum += microtime(true) - $st;
        $dt++;
    }
    $avg = $sum / $count;
    $w = max(strlen('sum'), strlen('count'), strlen('avg'));
    echo sprintf("%{$w}s: %s\n", 'sum', $sum);
    echo sprintf("%{$w}s: %s\n", 'count', $count);
    echo sprintf("%{$w}s: %s\n", 'avg', $avg);
} catch (Exception $e) {
    echo $e->getMessage() . "\n";
} finally {
    $dbh = null;
}

// login timeout: 15.019899845123
// connect: 0.043570995330811
// empty stored procedure: 0.02325701713562
