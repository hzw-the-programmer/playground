<?php
$dsn = 'sqlsrv:Server=10.0.37.84,1433;Database=db_not_exist';
$username = 'sa';
$password = 'kaifa@123';

try {
    $dbh = new PDO($dsn, $username, $password);
} catch (PDOException $e) {
    var_dump($e->getCode(), $e->getMessage());
}
