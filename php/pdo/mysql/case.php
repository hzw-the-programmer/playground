<?php
require 'config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);
    $dbh->exec("
        DROP PROCEDURE IF EXISTS p;
    ");
    $dbh->exec("
        CREATE PROCEDURE p()
        BEGIN
            DECLARE v INT DEFAULT 3;

            CASE v
                WHEN 2 THEN SELECT V;
                WHEN 3 THEN SELECT 0;
                ELSE
                    BEGIN
                        SELECT 100;
                    END;
            END CASE;
        END;
    ");

    $stmt = $dbh->query("CALL p();");
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    var_dump($rows);
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
