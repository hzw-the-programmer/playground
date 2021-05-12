<?php
include_once "../db_config.php";

if (count($argv) != 2) {
    echo "Usage: {$argv[0]} table\n";
    exit();
}

$table = $argv[1];

try {
    $dbh = new PDO(DSN, USERNAME, PASSWD);
    $sql = "SELECT * FROM $table";
    $stmt = $dbh->query($sql);
    while ($row = $stmt->fetch(PDO::FETCH_ASSOC)) {
        var_dump($row);
    }
} catch (PDOException $e) {
    echo $e->getMessage() . "\n";
} finally {
    $stmt = NULL;
    $dbh = NULL;
}
