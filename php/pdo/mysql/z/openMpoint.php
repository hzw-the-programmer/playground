<?php
require '../config.php';

try {
    $dbh = new PDO($dsn . ';charset=utf8mb4', $username, $passwd, $options);

    dump($dbh);

    $time = time();

    $dbh->exec("CALL openMpoint('19870123000001', 1, 0, 8, 5, '高阻1', $time, @mpid);");
    $dbh->exec("CALL openMpoint('19870123000001', 1, 1, 8, 5, '高阻2', $time, @mpid);");
    $dbh->exec("CALL openMpoint('19870123000001', 2, 0, 12, 5, '低阻1', $time, @mpid);");
    $dbh->exec("CALL openMpoint('19870123000001', 2, 1, 12, 5, '低阻2', $time, @mpid);");
    $dbh->exec("CALL openMpoint('19870123000001', 3, 0, 9, 5, '手圈1', $time, @mpid);");
    $dbh->exec("CALL openMpoint('19870123000001', 3, 1, 9, 5, '手圈2', $time, @mpid);");

    $dbh->exec("CALL openMpoint('19870123000002', 1, 0, 8, 6, '高阻1', $time, @mpid);");
    $dbh->exec("CALL openMpoint('19870123000002', 1, 1, 8, 6, '高阻2', $time, @mpid);");
    $dbh->exec("CALL openMpoint('19870123000002', 2, 0, 12, 6, '低阻1', $time, @mpid);");
    $dbh->exec("CALL openMpoint('19870123000002', 2, 1, 12, 6, '低阻2', $time, @mpid);");
    $dbh->exec("CALL openMpoint('19870123000002', 3, 0, 9, 6, '手圈1', $time, @mpid);");
    $dbh->exec("CALL openMpoint('19870123000002', 3, 1, 9, 6, '手圈2', $time, @mpid);");

    dump($dbh);
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}

function dump($dbh) {
    $stmt = $dbh->query("
        SELECT di.sn, ci.slot, ci.port, ci.type, p.name
        FROM mpoint AS mp
        LEFT JOIN channels_info AS ci
        ON mp.ciid = ci.id
        LEFT JOIN devices_info AS di
        ON ci.device_id = di.id
        LEFT JOIN place AS p
        ON mp.pid = p.id
    ");
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    var_dump($rows);
}
