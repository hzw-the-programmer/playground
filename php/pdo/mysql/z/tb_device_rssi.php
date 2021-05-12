<?php
function tb_device_rssi($dbh) {
    $dbh->exec("
        DROP TABLE IF EXISTS device_rssi;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS device_rssi(
            id INT AUTO_INCREMENT,
            device_id INT NOT NULL,
            rssi INT NOT NULL,
            updatetime INT NOT NULL,
            PRIMARY KEY (id)
        );
    ");
}
