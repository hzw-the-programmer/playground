<?php
include_once "../db_config.php";

try {
    $dbh = new PDO(DSN, USERNAME, PASSWD);
    $sql = "DELETE FROM channel_data";
    $stmt = $dbh->prepare($sql);
    $ret = $stmt->execute();
    if (!$ret) {
        print_r($stmt->errorInfo());
    }
} catch (PDOException $e) {
    echo $e->getMessage() . "\n";
} finally {
    $stmt = NULL;
    $dbh = NULL;
}
