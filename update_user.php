<?php
$id = $_GET['id'];
$firstname = $_POST['firstname'];
$lastname = $_POST['lastname'];
$phone = $_POST['phone'];
$email = $_POST['email'];

$conn = oci_connect('hzw', 'hzw', 'orcl', 'utf8');

$sql = sprintf("update users set firstname='%s', lastname='%s', phone='%s', email='%s' where id='%s'", $firstname, $lastname, $phone, $email, $id);
$sql = oci_parse($conn, $sql);
oci_execute($sql);
oci_free_statement($sql);

$sql = sprintf("select * from users where id='%s'", $id);
$sql = oci_parse($conn, $sql);
oci_execute($sql);
echo json_encode(array_change_key_case(oci_fetch_array($sql, OCI_ASSOC)));
oci_free_statement($sql);

oci_close($conn);
?>
