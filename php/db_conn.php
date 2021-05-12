<?php
include(dirname(__FILE__) . '/db_config.php');

$conn = sqlsrv_connect(
  $db_config['server_name'],
  ['Database' => $db_config['db_name'],
   'UID' => $db_config['user_name'],
   'PWD' => $db_config['password'],
   'CharacterSet' => 'UTF-8'
  ]
);

if(!$conn) {
  die(print_r(sqlsrv_errors(), true));
}
