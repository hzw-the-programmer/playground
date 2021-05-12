<?php
$a = ["name" => "Zhiwen He", "age" => 31];

echo sprintf("gettype: %s\n", gettype($a));
echo sprintf("is_array: %s\n", is_array($a) ? 'true' : 'false');

$b = $a;
$b["name"] = "Musk";
print_r($a);
print_r($b);

echo $a->name . "\n";
echo $a["name"] . "\n";
