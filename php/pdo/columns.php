<?php
$dsn = "sqlsrv:Server=192.168.1.123,1433;Database=test";
$username = "sa";
$passwd = "zhiwenhe";

try {
    $dbh = new PDO($dsn, $username, $passwd);
    $sql = "SELECT * FROM information_schema.columns WHERE table_name = 'channels'";
    $stmt = $dbh->query($sql);
    while ($row = $stmt->fetch(PDO::FETCH_ASSOC)) {
        var_dump($row);
    }
} catch (PDOException $e) {
    echo $e->getMessage() . "\n";
}
