<?php
function tb_mpoint_data($dbh) {
    $dbh->exec("
        DROP TABLE IF EXISTS mpoint_data;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS mpoint_data(
            id INT AUTO_INCREMENT,
            mpoint_id INT NOT NULL,
            swiftnum INT NOT NULL,
            data FLOAT NOT NULL,
            time INT NOT NULL,
            createtime INT NOT NULL,
            is_pad INT,
            PRIMARY KEY (id)
        );
    ");
}
