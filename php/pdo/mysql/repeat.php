<?php
require 'config.php';

$i = $argv[1];

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);
    $dbh->exec("
        DROP PROCEDURE IF EXISTS dorepeat;
    ");
    $dbh->exec("
        CREATE PROCEDURE dorepeat(i INT, OUT o INT)
        BEGIN
            SET o = 0;
            REPEAT
                SET o = o + 1;
            UNTIL o > i
            END REPEAT;
        END;
    ");

    $dbh->exec("CALL dorepeat($i, @o);");
    $stmt = $dbh->query("SELECT @o;");
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    var_dump($rows);
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
