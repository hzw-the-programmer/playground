<?php
echo "cwd: " . getcwd() . "\n";
echo __FILE__ . "\n";
$dir = dirname(__FILE__);
chdir($dir);
echo "cwd: " . getcwd() . "\n";
echo `ls -al` . "\n";
