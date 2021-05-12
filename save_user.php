<?php
$firstname = $_POST['firstname'];
$lastname = $_POST['lastname'];
$phone = $_POST['phone'];
$email = $_POST['email'];

$conn = oci_connect('hzw', 'hzw', 'orcl', 'utf8');

$sql = sprintf("insert into users values(users_seq.nextval, '%s', '%s', '%s', '%s')", $firstname, $lastname, $phone, $email);
$sql = oci_parse($conn, $sql);
oci_execute($sql);
oci_free_statement($sql);

$sql = sprintf("select * from users where firstname='%s' and lastname='%s' and phone='%s' and email='%s'", $firstname, $lastname, $phone, $email);
$sql = oci_parse($conn, $sql);
oci_execute($sql);
echo json_encode(array_change_key_case(oci_fetch_array($sql, OCI_ASSOC)));
oci_free_statement($sql);

oci_close($conn);
?>
