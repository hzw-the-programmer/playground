<?php
$serverName = 'localhost';
$connectionInfo = [
  'Database' => 'test',
  'UID' => 'sa',
  'PWD' => 'zhiwenhe',
  'CharacterSet' => 'UTF-8'];
$conn = sqlsrv_connect($serverName, $connectionInfo);
if (!$conn) {
  die(print_r(sqlsrv_errors(), true));
}

$sql = "
IF OBJECT_ID('testOutput', 'P') IS NOT NULL
DROP PROCEDURE testOutput
";
$stmt = sqlsrv_query($conn, $sql);
if (!$stmt) {
  die(print_r(sqlsrv_errors(), true));
}
sqlsrv_free_stmt($stmt);

$sql = "
CREATE PROCEDURE testOutput
@input nvarchar(50),
@output nvarchar(50) OUTPUT
AS
SELECT @output = @input
";
$stmt = sqlsrv_query($conn, $sql);
if (!$stmt) {
  die(print_r(sqlsrv_errors(), true));
}
sqlsrv_free_stmt($stmt);

$sql = '{call testOutput(?, ?)}';
$input = '你好世界！';
$output = '';
$params = [$input, [$output, SQLSRV_PARAM_INOUT, SQLSRV_PHPTYPE_STRING('UTF-8'), SQLSRV_SQLTYPE_NVARCHAR(50)]];
$stmt = sqlsrv_query($conn, $sql, $params);
if (!$stmt) {
  die(print_r(sqlsrv_errors(), true));
}
sqlsrv_free_stmt($stmt);

header('Content-type: text/javascript; charset=utf-8');
echo $output;

sqlsrv_close($conn);
