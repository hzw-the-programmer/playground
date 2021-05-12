<?php
$a = unpack('g', hex2bin('fa3ef642'))[1];
echo $a . "\n";

$a = unpack('G', hex2bin('42f63efa'))[1];
echo $a . "\n";

