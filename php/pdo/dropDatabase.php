<?php
//$dsn = 'sqlsrv:Server=10.0.37.84,1433;Database=test';
//3702 3701
$dsn = 'sqlsrv:Server=10.0.37.84,1433';
$username = 'sa';
$password = 'kaifa@123';

try {
    $dbh = new PDO($dsn, $username, $password);
    
    $sql = 'SELECT db_name() AS dbname';
    $stmt = $dbh->prepare($sql);
    if ($stmt->execute()) {
        var_dump($stmt->fetch(PDO::FETCH_ASSOC));
    } else {
        var_dump($stmt->errorCode(), $stmt->errorInfo());
    }

    $sql = 'DROP DATABASE test';
    $stmt = $dbh->prepare($sql);
    if ($stmt->execute()) {
    } else {
        var_dump($stmt->errorCode(), $stmt->errorInfo());
    }
} catch (PDOException $e) {
    var_dump($e);
}
