<?php
require 'config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);

    /*** t1 ***/
    $dbh->exec("
        SET foreign_key_checks = 0;
        DROP TABLE IF EXISTS device;
        SET foreign_key_checks = 1;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS device (
            id INT NOT NULL,
            sn VARCHAR(20) NOT NULL,
            INDEX (id)
        );
    ");

    /*** t2 ***/
    $dbh->exec("
        SET foreign_key_checks = 0;
        DROP TABLE IF EXISTS channel;
        SET foreign_key_checks = 1;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS channel (
            id INT NOT NULL,
            device_id INT NOT NULL,
            slot INT NOT NULL,
            INDEX (device_id),
            FOREIGN KEY (device_id) REFERENCES device (id)
        );
    ");

    $dbh->exec("
        DROP TABLE IF EXISTS t1;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS t1 (
            sn VARCHAR(20) NOT NULL,
            slot INT NOT NULL
        );
    ");

    $dbh->exec("
        DROP PROCEDURE IF EXISTS rcur1;
    ");
    $dbh->exec("
        CREATE PROCEDURE rcur1(device_id INT)
        BEGIN
            DECLARE sn VARCHAR(20);
            DECLARE slot INT;

            DECLARE done INT DEFAULT FALSE;

            DECLARE c1 CURSOR FOR
            SELECT d.sn, c.slot
            FROM device AS d
            LEFT JOIN channel AS c
            ON c.device_id = d.id
            WHERE d.id = device_id;

            DECLARE CONTINUE HANDLER FOR NOT FOUND SET done = TRUE;

            OPEN c1;

            l1: LOOP
                FETCH c1 INTO sn, slot;
                IF done THEN
                    LEAVE l1;
                END IF;
                INSERT INTO t1 VALUES (sn, slot);
            END LOOP;

            CLOSE c1;
        END;
    ");

    $dbh->exec("
        DROP PROCEDURE IF EXISTS rcur2;
    ");
    $dbh->exec("
        CREATE PROCEDURE rcur2()
        BEGIN
            DECLARE device_id INT;

            DECLARE done INT DEFAULT FALSE;

            DECLARE c1 CURSOR FOR
            SELECT id FROM device;

            DECLARE CONTINUE HANDLER FOR NOT FOUND SET done = TRUE;

            OPEN c1;

            l1: LOOP
                FETCH c1 INTO device_id;
                IF done THEN
                    LEAVE l1;
                END IF;
                CALL rcur1(device_id);
            END LOOP;

            CLOSE c1;
        END;
    ");

    $dbh->exec("
        INSERT INTO device VALUES (1, '19870123000001'), (2, '19870123000002');
    ");
    $dbh->exec("
        INSERT INTO channel VALUES (1, 1, 1), (2, 1, 2), (3, 2, 1), (4, 2, 2), (5, 1, 3), (6, 1, 4);
    ");

    $dbh->exec("CALL rcur2();");
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
