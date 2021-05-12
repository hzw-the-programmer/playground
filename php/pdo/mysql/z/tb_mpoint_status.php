<?php
function tb_mpoint_status($dbh) {
    $dbh->exec("
        DROP TABLE IF EXISTS mpoint_status;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS mpoint_status (
            id INT AUTO_INCREMENT,
            mpoint_id INT NOT NULL,
            swiftnum INT NOT NULL,
            raw_status INT NOT NULL,
            real_status INT NOT NULL,
            alarm_level INT NOT NULL,
            time INT NOT NULL,
            createtime INT NOT NULL,
            data FLOAT,
            PRIMARY KEY (id)
        );
    ");
}
