<?php
$filename = '/tmp/timer3';

$filesize = filesize($filename);
if ($filesize > 1 * 1024) {
    $file = fopen($filename, 'w');
    ftruncate($file, 0);
    fclose($file);
}
