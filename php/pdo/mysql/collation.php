<?php
require 'config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);

    $stmt = $dbh->query("
        SHOW COLLATION WHERE charset = 'utf8mb4';
    ");
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    var_dump($rows);
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
