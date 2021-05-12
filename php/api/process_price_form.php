<?php
header("Access-Control-Allow-Origin: http://localhost:3000");

if ($_SERVER['REQUEST_METHOD'] === 'OPTIONS') {
  header("Access-Control-Allow-Headers: Content-Type");
  return;
}

$values = file_get_contents("php://input");
$values = json_decode($values);
$values->price->number++;

//header('Content-type: application/json');
//file_put_contents("php://output", json_encode($price));
echo json_encode($values);
