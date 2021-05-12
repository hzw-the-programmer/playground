<?php
require 'config.php';

$n = $argv[1];
$m = $argv[2];

try {
    $s = microtime(true);
    $dbh = new PDO($dsn, $username, $passwd, $options);
    $dbh->exec("
        DROP FUNCTION IF EXISTS VerboseCompare;
    ");
    $dbh->exec("
        CREATE FUNCTION VerboseCompare(n INT, m INT)
        RETURNS VARCHAR(50)
        BEGIN
            DECLARE s VARCHAR(50);

            IF n = m THEN SET s = 'equals';
            ELSE
                IF n > m THEN SET s = 'greater';
                ELSE SET s = 'less';
                END IF;

                SET s = CONCAT('is ', s, ' than');
            END IF;

            SET s = CONCAT(n, ' ', s, ' ', m, '.');

            RETURN s;
        END;
    ");

    $stmt = $dbh->query("SELECT VerboseCompare($n, $m);");
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    echo (microtime(true) - $s) . PHP_EOL;
    var_dump($rows);
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
