<?php
require 'config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);

    $stmt = $dbh->query("
        SHOW CHARACTER SET WHERE maxlen = 4;
    ");
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    var_dump($rows);
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
