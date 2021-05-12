<?php
require '../config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);

    $time = time();

    //$dbh->exec("CALL statusData('19870123000001', 2, 0, 12, $time, $time, 123, 0x21, 0x20, 0x01, 1.23);");
    //$dbh->exec("CALL statusData('19870123000001', 2, 0, 12, $time, $time, 0, 261, 261, 0, 1.23);");
    //$dbh->exec("CALL statusData('19870123000001', 2, 0, 12, $time, $time, 0, 260, 260, 0, 1.23);");
    //$dbh->exec("CALL statusData('19870123000001', 2, 0, 12, $time, $time, 0, 259, 259, 0, 1.23);");
    $dbh->exec("CALL statusData('19870123000001', 2, 0, 12, $time, $time, 124, 0x22, 0x20, 0x02, 1.24);");
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
