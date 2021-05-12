<?php
if ($argc != 2) {
  die("Usage $argv[0] file");
}

$handle = fopen($argv[1], 'r');

$pre = null;
while (($line = fgets($handle)) != false) {
  $a = explode(' ', $line);
  
  if ($pre == null) {
    $pre = new DateTime("$a[0] $a[1]");
    continue;
  }

  $cur = new DateTime("$a[0] $a[1]");
  $diff = $cur->gettimestamp() - $pre->gettimestamp();
  if ($diff != 60) {
    echo $diff . " " . $pre->format('Y-m-d H:i:s') . " " . $cur->format('Y-m-d H:i:s') . "\n";
  }
  $pre = $cur;
}
