<?php
$dsn = "sqlsrv:Server={$argv[1]};Database=test;LoginTimeout=1";
$user = $argv[2];
$passwd = $argv[3];

$dbh = new PDO($dsn, $user, $passwd);
try {
    echo "before sleep\n";
    //sleep(10);
    echo "before transaction\n";
    $dbh->beginTransaction();

    echo "before sleep\n";
    sleep(10);
    echo "before query\n";
    $dbh->query('SELECT 1');

    echo "before sleep\n";
    sleep(10);
    echo "before commit\n";
    $dbh->commit();
} catch (PDOException $e) {
    if ($dbh->inTransaction()) {
        echo "before rollBack\n";
        $dbh->rollBack();
    }

    $err = $e->errorInfo;
    echo sprintf("%s, %s, %s\n", $err[0], $err[1], $err[2]);
}
