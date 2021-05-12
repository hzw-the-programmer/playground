<?php
require '../db_config.php';

try {
    $start = microtime(true);
    $dbh = new PDO(DSN . ";LoginTimeout=5", USERNAME, PASSWD); //default 15s
    //$dbh->setAttribute(PDO::SQLSRV_ATTR_QUERY_TIMEOUT, 10);
    echo "before sleep\n";
    sleep(20);
    echo "after sleep\n";
    $s = microtime(true);
    $stmt = $dbh->prepare("SELECT * FROM channel_data");
    echo "prepare: " . (microtime(true) - $s) . "\n";
    var_dump($stmt);
    if ($stmt) {
        //$stmt->setAttribute(PDO::SQLSRV_ATTR_QUERY_TIMEOUT, 10);
        $s = microtime(true);
        $ret = $stmt->execute();
        echo (microtime(true) - $s) . "\n";
        if (!$ret) {
            print_r($stmt->errorInfo());
        }
        var_dump($stmt);
        var_dump($stmt->fetchAll());
    }
} catch (PDOException $e) {
    echo $e->getMessage() . "\n";
} finally {
    echo (microtime(true) - $start) . "\n";
}
