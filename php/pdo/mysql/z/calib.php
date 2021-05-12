<?php
require '../config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);

    $time = time();
    $dbh->exec("CALL calib('19870123000001', 4, 0, 8, $time, $time, 111, 0x20, 100);");
    //$dbh->exec("CALL calib('19870123000010', 4, 0, 8, $time, $time, 111, 0x20, 100);");
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
