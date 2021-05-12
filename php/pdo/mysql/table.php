<?php
require 'config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);

    $dbh->exec("
        DROP TABLE IF EXISTS devices_info;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS devices_info (
            id INT AUTO_INCREMENT,
            sn VARCHAR(20) NOT NULL,
            ip CHAR(15),
            port INT,
            devaddr VARCHAR(20),
            version VARCHAR(10),
            type VARCHAR(10),
            createtime INT,
            updatetime INT,
            parent_id INT,
            level INT,
            PRIMARY KEY (id)
        );
    ");

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
            createtime INT,
            updatetime INT,
            PRIMARY KEY (id)
        );
    ");

    $dbh->exec("
        DROP TABLE IF EXISTS place;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS place (
            id INT AUTO_INCREMENT,
            name VARCHAR(20) NOT NULL,
            level INT NOT NULL,
            pid INT NOT NULL,
            PRIMARY KEY (id)
        );
    ");

    $dbh->exec("
        DROP TABLE IF EXISTS mpoint;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS mpoint (
            id INT AUTO_INCREMENT,
            name VARCHAR(20) NOT NULL,
            pid INT NOT NULL,
            ciid INT NOT NULL,
            starttime INT NOT NULL,
            endtime INT NOT NULL,
            PRIMARY KEY (id)
        );
    ");

    $dbh->exec("
        DROP TABLE IF EXISTS mpoint_realtime_status;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS mpoint_realtime_status (
            id INT AUTO_INCREMENT,
            mpoint_id INT NOT NULL,
            swiftnum INT NOT NULL,
            raw_status INT NOT NULL,
            real_status INT NOT NULL,
            alarm_level INT NOT NULL,
            time INT NOT NULL,
            createtime INT NOT NULL,
            data float NOT NULL,
            PRIMARY KEY (id)
        );
    ");

    $dbh->exec("
        DROP TABLE IF EXISTS mpoint_pre_status;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS mpoint_pre_status (
            id INT AUTO_INCREMENT,
            mpoint_id INT NOT NULL,
            swiftnum INT NOT NULL,
            raw_status INT NOT NULL,
            real_status INT NOT NULL,
            alarm_level INT NOT NULL,
            time INT NOT NULL,
            createtime INT NOT NULL,
            data float NOT NULL,
            PRIMARY KEY (id)
        );
    ");
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
