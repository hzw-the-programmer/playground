<?php
$a = pack('J', null);
echo bin2hex($a) . "\n";

$a = pack('J', '');
echo bin2hex($a) . "\n";

$a = pack('J', 20170123000001);
echo bin2hex($a) . "\n";

$a = pack('J', 0x00001258390438c1);
echo bin2hex($a) . "\n";

$a = pack('J', '20170123000001');
echo bin2hex($a) . "\n";

$a = pack('P', '20170123000001');
echo bin2hex($a) . "\n";
