<?php
require 'config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);
    $dbh->exec("
        DROP FUNCTION IF EXISTS SimpleCompare;
    ");
    $dbh->exec("
        CREATE FUNCTION SimpleCompare(n INT, m INT)
        RETURNS VARCHAR(20)
        BEGIN
            DECLARE s VARCHAR(20);

            IF n > m THEN SET s = '>';
            ELSEIF n = m THEN SET s = '=';
            ELSE SET s = '<';
            END IF;

            SET s = CONCAT(n, ' ', s, ' ', m);

            RETURN s;
        END;
    ");

    $stmt = $dbh->query("SELECT SimpleCompare(2, 1);");
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    var_dump($rows);
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
