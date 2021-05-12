<?php
require 'config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);

    $dbh->exec("
        CREATE OR REPLACE VIEW v1 AS
        SELECT REPEAT('a', 2)
        UNION SELECT REPEAT('b', 4)
        UNION SELECT REPEAT('c', 8);
    ");

    $stmt = $dbh->query('SELECT * FROM v1;');
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    var_dump($rows);
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
