<?php
$serverName = "192.168.1.123";
$connectionOptions = array(
    "Database" => "zserver",
    "Uid" => "sa",
    "PWD" => "hzw"
);
//Establishes the connection
$conn = sqlsrv_connect($serverName, $connectionOptions);
if($conn)
    echo "Connected!\n";
else
    print_r(sqlsrv_errors());
?>
