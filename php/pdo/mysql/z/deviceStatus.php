<?php
require '../config.php';

try {
    $dbh = new PDO($dsn, $username, $passwd, $options);

    $time = time();
    $poweroff = 261;
    $poweron = 260;
    $online = 259;
    //$dbh->exec("CALL deviceStatus('19870123000001', $time, $poweroff);");
    //$dbh->exec("CALL deviceStatus('19870123000001', $time, $poweron);");
    $dbh->exec("CALL deviceStatus('19870123000001', $time, $online);");
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
