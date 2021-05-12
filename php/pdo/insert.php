<?php
include_once "../db_config.php";

try {
    $dbh = new PDO(DSN, USERNAME, PASSWD);
    $sql = "INSERT INTO channel_data (cid, pid, ctime, data) VALUES (:cid, :pid, :ctime, :data)";
    $stmt = $dbh->prepare($sql);
    $stmt->bindParam(":cid", $cid);
    $stmt->bindParam(":pid", $pid);
    $stmt->bindParam(":ctime", $ctime);
    $stmt->bindParam(":data", $data);
    $cid = 1;
    $pid = 1;
    $ctime = "2018-01-23 00:00:00";
    $data = 123.123;
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