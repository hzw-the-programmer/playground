<?php
$bin = hex2bin('ffffffffffffffff');
echo unpack('J', $bin)[1] . PHP_EOL;
echo sprintf('%014u', unpack('J', $bin)[1]) . PHP_EOL;
echo bin2hex(pack('J', -1)) . PHP_EOL;
