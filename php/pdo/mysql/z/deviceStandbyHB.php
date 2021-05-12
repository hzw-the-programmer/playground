<?php
require '../config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);

    $time = time();
    $dbh->exec("CALL deviceStandbyHB('19870123000001', $time);");
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
