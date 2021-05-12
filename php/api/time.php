<?php
$t = time();
echo date('Y-m-d H:i:s', $t) . PHP_EOL;
$t = ($t + 60) - $t % 60;
echo date('Y-m-d H:i:s', $t) . PHP_EOL;
