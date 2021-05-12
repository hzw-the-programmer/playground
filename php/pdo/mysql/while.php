<?php
require 'config.php';

$c = $argv[1];
$n = $argv[2];

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);
    $dbh->exec("
        DROP PROCEDURE IF EXISTS while1;
    ");
    $dbh->exec("
        CREATE PROCEDURE while1(c CHAR(1), n INT, OUT s VARCHAR(50))
        BEGIN
            SET s = '';
            WHILE n > 0 DO
                SET n = n - 1;
                SET s = CONCAT(c, s);
            END WHILE;
        END;
    ");

    $dbh->exec("CALL while1('$c', $n, @s);");
    $stmt = $dbh->query("SELECT @s;");
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    var_dump($rows);
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
