<?php
require 'config.php';

try {
    $s = microtime(true);
    $dbh = new PDO($dsn, $username, $passwd, $options);
    $dbh->exec("
        DROP PROCEDURE IF EXISTS doiterate;
    ");
    $dbh->exec("
        CREATE PROCEDURE doiterate(INOUT n INT)
        BEGIN
            label: LOOP
                SET n = n + 1;
                IF n < 10000 THEN
                    ITERATE label;
                END IF;
                LEAVE label;
            END LOOP label;
        END;
    ");

    $dbh->exec("SET @n = 0;");
    $dbh->exec("CALL doiterate(@n);");
    $stmt = $dbh->query("SELECT @n;");
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    echo (microtime(true) - $s) . PHP_EOL;
    var_dump($rows);
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
