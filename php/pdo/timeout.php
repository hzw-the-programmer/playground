<?php
$server = '10.0.37.248'; //$argv[1];
$user = 'sa'; //$argv[2];
$passwd = '123456'; //$argv[3];

$dsn = "sqlsrv:Server=$server;Database=test;LoginTimeout=5";

$dbh = new PDO($dsn, $user, $passwd, [
    PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION,
    PDO::SQLSRV_ATTR_QUERY_TIMEOUT => 10 // this does not work as I expected.
]);

echo "before sleep\n"; // after this display, I disable the network interface: ifconfig enp0s3 down.
sleep(10);
echo "after sleep\n"; // when this display, network interface is already down.

$start = microtime(true);
try {
    echo "before query\n"; // this will display.
    $dbh->query("WAITFOR DELAY '00:00:5'; SELECT 1"); // it blocks here for about 242 seconds.
} catch (PDOException $e) {
    print_r($e->errorInfo); // this will run after about 242 seconds, not as I expected 1 second.
}
echo "cost: " . (microtime(true) - $start) . PHP_EOL;
