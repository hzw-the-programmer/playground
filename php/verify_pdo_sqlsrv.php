<?php
$dsn = "sqlsrv:Server=192.168.1.123;Database=zserver";
$user = "sa";
$password = "hzw";

try {
    $dbh = new PDO($dsn, $user, $password);
    echo "Connected!\n";
} catch (PDOException $e) {
    echo 'Connection failed: ' . $e->getMessage() . "\n";
}
