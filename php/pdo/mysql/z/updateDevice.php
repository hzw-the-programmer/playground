<?php
require '../config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);

    $time = time();
    //$dbh->exec("CALL updateDevice('19870123000004', '', '', 0, '0001', $time);");
    $dbh->exec("CALL updateDevice('19870123000001', '', '', 0, '0001', $time);");
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
