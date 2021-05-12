<?php
if ($argc !== 5) {
    die("{$argv[0]} server user pd db\n");
}
$server = $argv[1];
$user = $argv[2];
$pd = $argv[3];
$db = $argv[4];

$dsn = "sqlsrv:Server=$server;Database=$db";

try {
    $dbh = new \PDO($dsn, $user, $pd);

    $stmt = $dbh->prepare('
        SELECT sn, ip FROM devices_info;
        SELECT slot, port, type FROM channels_info;
    ');

    if (!$stmt->execute()) {
        die(implode($stmt->errorInfo(), ', ') . PHP_EOL);
    }

    do {
        $count = 0;
        while ($row = $stmt->fetch(PDO::FETCH_ASSOC)) {
            $count++;
            //var_dump($row);
        }
        echo "$count\n";
    } while ($stmt->nextRowset());
} catch (\PDOException $e) {
    die($e->getMessage() . PHP_EOL);
}
