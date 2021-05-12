<?php
function tb_mpoint($dbh) {
    $dbh->exec("
        DROP TABLE IF EXISTS mpoint;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS mpoint (
            id INT AUTO_INCREMENT,
            pid INT NOT NULL,
            ciid INT NOT NULL,
            name VARCHAR(20) COLLATE utf8mb4_unicode_ci NOT NULL,
            starttime INT NOT NULL,
            endtime INT NOT NULL,
            PRIMARY KEY (id)
        );
    ");
}
