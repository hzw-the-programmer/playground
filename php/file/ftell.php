<?php
$f = fopen('DataManager.php', 'a');
fseek($f, 0, SEEK_END);
echo ftell($f) . PHP_EOL;
fclose($f);

$f = fopen('DataManager.php', 'a+');
fseek($f, 0, SEEK_END);
echo ftell($f) . PHP_EOL;
fclose($f);
