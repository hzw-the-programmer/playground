<?php
require '../config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);

    $time = time();
    //$dbh->exec("CALL hbUpdateAvgData('19870123000001', 1, 0, 8, $time, $time, 111, 12.3);");
    $dbh->exec("CALL hbUpdateAvgData('19870123000001', 1, 2, 8, $time, $time, 111, 12.3);");
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
