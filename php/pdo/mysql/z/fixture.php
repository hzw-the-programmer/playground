<?php
require '../config.php';

try {
    $dbh = new PDO($dsn . ';charset=utf8mb4', $username, $passwd, $options);
    data_device($dbh);
    data_place($dbh);
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}

function data_device($dbh) {
    $time = time();
    $dbh->exec("CALL updateDevice('19870123000001', '', '', 0, '0001', $time);");
    $dbh->exec("CALL updateDevice('19870123000002', '', '', 0, '0002', $time);");
    $dbh->exec("CALL updateDevice('19870123000003', '', '', 0, '0003', $time);");
}

function data_place($dbh) {
    $time = time();
    $dbh->exec("
        INSERT INTO place
        (pid, name, level)
        VALUES
        (0, '工厂', 1),
        (1, '车间', 2),
        (2, '区域', 3),
        (3, '线体', 4),
        (4, '工位1', 5),
        (4, '工位2', 5);
    ");
}
