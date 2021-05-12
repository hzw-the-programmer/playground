<?php
$a = [1, 2, 3];
$l = array_push($a, 4, 5, 6);
echo "$l\n";
print_r($a);

while ($e = array_pop($a)) {
    echo "$e\n";
}
print_r($a);
