<?php
require 'config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);

    /*** t1 ***/
    $dbh->exec("
        DROP TABLE IF EXISTS t1;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS t1 (
            id CHAR(16),
            data INT
        );
    ");

    /*** t2 ***/
    $dbh->exec("
        DROP TABLE IF EXISTS t2;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS t2 (
            data INT
        );
    ");

    /*** t3 ***/
    $dbh->exec("
        DROP TABLE IF EXISTS t3;
    ");
    $dbh->exec("
        CREATE TABLE IF NOT EXISTS t3 (
            id CHAR(16),
            data INT
        );
    ");

    /*** curdemo ***/
    $dbh->exec("
        DROP PROCEDURE IF EXISTS curdemo;
    ");
    $dbh->exec("
        CREATE PROCEDURE curdemo()
        BEGIN
            DECLARE id1 CHAR(16);
            DECLARE data1, data2 INT;
            DECLARE done INT DEFAULT FALSE;

            DECLARE c1 CURSOR FOR SELECT id, data FROM t1;
            DECLARE c2 CURSOR FOR SELECT data FROM t2;

            DECLARE CONTINUE HANDLER FOR NOT FOUND SET done = TRUE;

            OPEN c1;
            OPEN c2;

            read_loop: LOOP
                FETCH c1 INTO id1, data1;
                FETCH c2 INTO data2;

                IF done THEN
                    LEAVE read_loop;
                END IF;

                IF data1 > data2 THEN
                    INSERT INTO t3 VALUES (id1, data1);
                ELSE
                    INSERT INTO t3 VALUES (id1, data2);
                END IF;
            END LOOP read_loop;

            CLOSE c1;
            CLOSE c2;
        END;
    ");

    $dbh->exec("
        INSERT INTO t1 VALUES ('01', 1), ('02', 4);
    ");
    $dbh->exec("
        INSERT INTO t2 VALUES (2), (3);
    ");
    
    $stmt = $dbh->query("SELECT id, data FROM t3;");
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    var_dump($rows);
    
    $dbh->exec("CALL curdemo();");

    $stmt = $dbh->query("SELECT id, data FROM t3;");
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    var_dump($rows);
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
