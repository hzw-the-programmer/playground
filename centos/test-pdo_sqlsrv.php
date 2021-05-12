<?php
if ($argc !== 4) {
    die("{$argv[0]} server user passwd\n");
}
$server = $argv[1];
$user = $argv[2];
$passwd = $argv[3];
$dbh = new PDO("sqlsrv:Server=$server", $user, $passwd);
$dbh->query('SELECT 1');
