<?
$serverName = "10.0.22.48";
$connectionInfo = array("UID" => "sa", "PWD" => "123456789", "Database" => "kaifa","CharacterSet"=>"UTF-8");
$conn = sqlsrv_connect($serverName, $connectionInfo);
if ($conn) {
  echo "success!";
} else {
  echo "fail!";
}
?>
