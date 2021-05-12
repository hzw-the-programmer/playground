<?php
$dsn = 'sqlsrv:Server=10.0.37.84,1433';
$database = ';Database=test';
$username = 'sa';
$password = 'kaifa@123';

begin:
try {
    $dbh = new PDO($dsn . $database, $username, $password);
} catch (PDOException $e) {
    if ($e->errorInfo[1] == 4060) {
        $dbh = new PDO($dsn, $username, $password);
        
        $sql = 'CREATE DATABASE test';
        $stmt = $dbh->prepare($sql);
        if (!$stmt->execute()) {
            var_dump($stmt->errorCode(), $stmt->errorInfo());
            exit();
        }
        goto begin;
    } else {
        var_dump($e);
    }
}

/*
try {
    $dbh = new PDO($dsn, $username, $password);
    
    $sql = 'SELECT db_name() AS dbName';
    $stmt = $dbh->prepare($sql);
    if (!$stmt->execute()) {
        var_dump($stmt->errorCode(), $stmt->errorInfo());
        exit();
    }
    echo "dbName: {$stmt->fetch(PDO::FETCH_ASSOC)['dbName']}\n";

    $sql = "SELECT DB_ID(N'test') AS dbExists";
    $stmt = $dbh->prepare($sql);
    if (!$stmt->execute()) {
        var_dump($stmt->errorCode(), $stmt->errorInfo());
        exit();
    }
    $dbExists = $stmt->fetch(PDO::FETCH_ASSOC)['dbExists'];
    if (!$dbExists) {
        $sql = 'CREATE DATABASE test';
        $stmt = $dbh->prepare($sql);
        if (!$stmt->execute()) {
            var_dump($stmt->errorCode(), $stmt->errorInfo());
            exit();
        }
    } else {
        echo "dbExists\n";
    }


} catch (PDOException $e) {
    echo $e->getMessage() . "\n";
}
*/
