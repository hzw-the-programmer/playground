<?php
require '../config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);

    $time = time();
    $dbh->exec("CALL closeMpoint(1, $time);");
    $dbh->exec("CALL closeMpoint(2, $time);");
} catch(PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
