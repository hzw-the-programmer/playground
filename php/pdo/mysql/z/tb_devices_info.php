<?php
function tb_devices_info($dbh) {
    $dbh->exec("
        DROP TABLE IF EXISTS devices_info;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS devices_info (
            id INT AUTO_INCREMENT,
            parent_id INT NOT NULL,
            sn VARCHAR(20) NOT NULL,
            mac VARCHAR(20),
            ip CHAR(15) NOT NULL,
            type VARCHAR(10) NOT NULL,
            status CHAR(1) NOT NULL,
            level INT NOT NULL,
            createtime INT NOT NULL,
            updatetime INT NOT NULL,
            devaddr VARCHAR(20),
            port INT,
            version VARCHAR(10),
            PRIMARY KEY (id)
        );
    ");
}
