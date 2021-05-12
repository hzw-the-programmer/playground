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

    $dbh->exec("
        SET foreign_key_checks = 0;
        DROP TABLE IF EXISTS channel;
        SET foreign_key_checks = 1;
    ");
    /*
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS channel (
            id INT NOT NULL,
            device_id INT NOT NULL,
            slot INT not NULL,
            INDEX (device_id),
            FOREIGN KEY (device_id) REFERENCES device (id)
        );
    ");
    */
    /*
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS channel (
            id INT NOT NULL,
            device_id INT NOT NULL,
            slot INT not NULL,
            INDEX (device_id),
            FOREIGN KEY (device_id) REFERENCES device (id) ON DELETE CASCADE
        );
    ");
    */
    /*
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS channel (
            id INT NOT NULL,
            device_id INT NOT NULL,
            slot INT not NULL,
            INDEX (device_id),
            FOREIGN KEY (device_id) REFERENCES device (id) ON UPDATE CASCADE
        );
    ");
    */
    /*
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS channel (
            id INT NOT NULL,
            device_id INT,
            slot INT not NULL,
            INDEX (device_id),
            FOREIGN KEY (device_id) REFERENCES device (id) ON DELETE SET NULL
        );
    ");
    */
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS channel (
            id INT NOT NULL,
            device_id INT,
            slot INT not NULL,
            INDEX (device_id),
            FOREIGN KEY (device_id) REFERENCES device (id) ON UPDATE SET NULL
        );
    ");

    $dbh->exec("
        INSERT INTO device VALUES (1, '19870123000001'), (2, '19870123000002');
    ");
    /*
    $dbh->exec("
        INSERT INTO channel VALUES (1, 3, 1), (2, 4, 2);
    ");
    */
    $dbh->exec("
        INSERT INTO channel VALUES (1, 1, 1), (2, 1, 2), (3, 2, 1), (4, 2, 2);
    ");

    $stmt = $dbh->query("SELECT * FROM device;");
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    var_dump($rows);

    $stmt = $dbh->query("SELECT * FROM channel;");
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    var_dump($rows);

    //$dbh->exec("DELETE FROM device WHERE id = 1;");
    $dbh->exec("UPDATE device SET id = 4 WHERE id = 1;");
    
    echo str_repeat("*", 40) . "\n";

    $stmt = $dbh->query("SELECT * FROM device;");
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    var_dump($rows);

    $stmt = $dbh->query("SELECT * FROM channel;");
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    var_dump($rows);
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
