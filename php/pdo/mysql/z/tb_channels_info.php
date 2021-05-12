<?php
function tb_channels_info($dbh) {
    $dbh->exec("
        DROP TABLE IF EXISTS channels_info;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS channels_info (
            id INT AUTO_INCREMENT,
            device_id INT NOT NULL,
            slot INT NOT NULL,
            port INT NOT NULL,
            type INT NOT NULL,
            createtime INT NOT NULL,
            updatetime INT NOT NULL,
            PRIMARY KEY (id)
        );
    ");
}
