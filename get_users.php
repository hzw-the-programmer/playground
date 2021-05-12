<?php
$result = array();

$conn = oci_connect('hzw', 'hzw', 'orcl', 'utf8');

$sql = 'select count(*) from users';
$sql = oci_parse($conn, $sql);
oci_execute($sql, OCI_DEFAULT);
$row = oci_fetch_row($sql);
$result['total'] = $row[0];
oci_free_statement($sql);

$sql = 'select * from users';
$sql = oci_parse($conn, $sql);
oci_execute($sql);
$rows = array();
while ($array = oci_fetch_array($sql, OCI_ASSOC)) {
  array_push($rows, array_change_key_case($array));
}
$result['rows'] = $rows;
oci_free_statement($sql);

oci_close($conn);

echo json_encode($result)
?>
