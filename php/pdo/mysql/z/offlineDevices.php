<?php
require '../config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);

    $time = time();
    $dbh->exec("CALL offlineDevices($time);");
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
