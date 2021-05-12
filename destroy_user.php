<?php
$id = $_POST['id'];

$conn = oci_connect('hzw', 'hzw', 'orcl', 'utf8');

$sql = sprintf("delete from users where id='%s'", $id);
$sql = oci_parse($conn, $sql);
oci_execute($sql);
oci_free_statement($sql);

$result['success']=true;
echo json_encode($result);

oci_close($conn);
?>
