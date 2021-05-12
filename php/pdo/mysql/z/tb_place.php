<?php
function tb_place($dbh) {
    $dbh->exec("
        DROP TABLE IF EXISTS place;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS place (
            id INT AUTO_INCREMENT,
            pid INT NOT NULL,
            name VARCHAR(20) COLLATE utf8mb4_unicode_ci NOT NULL,
            level INT NOT NULL,
            del INT,
            PRIMARY KEY (id)
        );
    ");
}
