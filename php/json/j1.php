<?php
$a = ['cmd' => 'join', 'cmdseq' => 1, 'appeui' => '4B46415050000001'];
echo json_encode($a) . PHP_EOL;

$a = '\x0a\x01\x02';
echo bin2hex($a) . PHP_EOL;

$a = "\x0a\x01\x02";
echo bin2hex($a) . PHP_EOL;
