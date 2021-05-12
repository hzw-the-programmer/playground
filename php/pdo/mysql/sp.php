<?php
require 'config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);

    $dbh->exec('
        DROP PROCEDURE IF EXISTS dorepeat;
        CREATE PROCEDURE dorepeat(INOUT p1 INT, IN p2 INT)
        BEGIN
            SET p1 = p1 + p2;
        END;
    ');

    for ($i = 0, $j = 10; $i < 1000; $i++, $j++) {
        $dbh->exec("
            SET @p1 = $i, @p2 = $j;
            CALL dorepeat(@p1, @p2);
        ");

        $stmt = $dbh->query('
            SELECT @p1, @p2;
        ');

        $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
        //$rows = $stmt->fetch(PDO::FETCH_ASSOC);

        var_dump($rows);
    }
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
