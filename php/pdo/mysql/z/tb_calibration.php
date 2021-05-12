<?php
function tb_calibration($dbh) {
    $dbh->exec("
        DROP TABLE IF EXISTS calibration;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS calibration(
            id INT AUTO_INCREMENT,
            cid INT NOT NULL,
            time INT NOT NULL,
            result INT NOT NULL,
            data FLOAT NOT NULL,
            PRIMARY KEY (id)
        );
    ");
}
