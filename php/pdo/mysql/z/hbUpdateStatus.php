<?php
require '../config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);

    $time = time();
    /*
    $dbh->exec("
        CALL hbUpdateStatus(
            '19870123000001', 1, 0, 8,
            $time, $time, 123, 0x21, 0x20, 0x01, 1.23
        );
    ");
    */
    $dbh->exec("
        CALL hbUpdateStatus(
            '19870123000001', 1, 0, 8,
            $time - 1200, $time - 1200, 121, 0x00, 0x00, 0x00, 234
        );
    ");
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
