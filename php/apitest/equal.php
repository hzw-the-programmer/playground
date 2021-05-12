<?php

echo '0 == false: ' . (0 == false ? 'yes' : 'no') . "\n";
echo '0.0 == false: ' . (0.0 == false ? 'yes' : 'no') . "\n";
echo '"0" == false: ' . ("0" == false ? 'yes' : 'no') . "\n";
echo '"0.0" == false: ' . ("0.0" == false ? 'yes' : 'no') . "\n";
echo '"" == false: ' . ("" == false ? 'yes' : 'no') . "\n";
echo '[] == false: ' . ([] == false ? 'yes' : 'no') . "\n";
echo 'null == false: ' . (null == false ? 'yes' : 'no') . "\n";
echo '$a == false: ' . ($a == false ? 'yes' : 'no') . "\n";

echo "\n";

echo 'is 0 empty: ' . (empty(0) ? 'yes' : 'no') . "\n";
echo 'is 0.0 empty: ' . (empty(0.0) ? 'yes' : 'no') . "\n";
echo 'is "0" empty: ' . (empty("0") ? 'yes' : 'no') . "\n";
echo 'is "0.0" empty: ' . (empty("0.0") ? 'yes' : 'no') . "\n";
echo 'is "" empty: ' . (empty("") ? 'yes' : 'no') . "\n";
echo 'is [] empty: ' . (empty([]) ? 'yes' : 'no') . "\n";
echo 'is null empty: ' . (empty(null) ? 'yes' : 'no') . "\n";
echo 'is $a empty: ' . (empty($a) ? 'yes' : 'no') . "\n";

echo "\n";

$str = 'haha';
echo "'haha' == 'haha': " . ($str == 'haha' ? 'yes' : 'no') . "\n";
