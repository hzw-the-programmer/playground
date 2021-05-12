<?php
$pid = isset($_GET['pid']) ? $_GET['pid'] : '';
$page = isset($_GET['page']) ? $_GET['page'] : '';
$rows = isset($_GET['rows']) ? $_GET['rows'] : '';

if ($pid === '') {
  $pid = 1;
}
if ($page === '') {
  $page = 1;
}
if ($rows === '') {
  $rows = 20;
}

include('../db/db_conn.php');
include('../utils/utils.php');

$sql = '{CALL get_devices_under_place(?, ?, ?, ?)}';
$total = 0;
$params = [$pid, $page, $rows, [$total, SQLSRV_PARAM_OUT]];
$stmt = sqlsrv_query($conn, $sql, $params);
if (!$stmt) {
  die(print_r(sqlsrv_errors(), true));
}
$results['total'] = $total;
$results['rows'] = [];
while($row = sqlsrv_fetch_array($stmt, SQLSRV_FETCH_ASSOC)) {
  $results['rows'][] = $row;
}
sqlsrv_free_stmt($stmt);
sqlsrv_close($conn);

// echoJson($results);
header('Content-type: text/javascript; charset=utf-8');
echo json_encode($array, JSON_PRETTY_PRINT | JSON_UNESCAPED_UNICODE);
