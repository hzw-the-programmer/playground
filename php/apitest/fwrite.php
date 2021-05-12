<?php
$f = fopen('test', 'a');
flock($f, LOCK_EX);
fwrite($f, 'haha');
sleep(20);
fwrite($f, 'hehe');
flock($f, LOCK_UN);
