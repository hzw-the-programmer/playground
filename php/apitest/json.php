<?php
$a = ['a' => 1, 'b' => 2];
$b = [1, 2];
echo json_encode($a) . PHP_EOL;
echo json_encode($b) . PHP_EOL;
$a1 = ['ip' => '192.168.1.2', 'port' => 9501];
$a2 = ['devaddr' => '0001'];
$address = [$a1, $a2];
echo json_encode($address) . PHP_EOL;
