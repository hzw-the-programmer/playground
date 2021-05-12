<?php
$dbh = new PDO('sqlsrv:Server=10.0.37.90;Database=test', 'sa', 'pw');
$stmt = $dbh->prepare('INSERT INTO co (x, y) VALUES (?, ?)');
// var_dump($stmt);
// var_dump($argv);
$fn = $argv[1];
// echo $fn;
$handle = fopen($fn, 'r');
// var_dump(fgetcsv($handle));
// var_dump(fgetcsv($handle));
// var_dump(fgetcsv($handle));
while ($a = fgetcsv($handle)) {
  // var_dump($a);
  if (!$stmt->execute([$a[1], $a[2]])) {
    print_r($stmt->errorInfo());
  }
}