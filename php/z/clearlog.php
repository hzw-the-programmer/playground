<?php
require __DIR__ . '/vendor/autoload.php';

use Hzw\Config;

$dh = opendir(Config::LOG_DIR);
if (!$dh) die("open log dir failed\n");

while (($filename = readdir($dh)) !== false) {
    if ($filename === '.'
        || $filename === '..'
        || $filename === 'touch'
    ) {
        continue;
    }
    $filename = Config::LOG_DIR . "/$filename";
    $filesize = filesize($filename);
    if ($filesize > Config::LOG_FILE_MAX_SIZE) {
        $file = fopen($filename, 'w');
        if ($file) {
            ftruncate($file, 0);
            fclose($file);
        }
    }
}

closedir($dh);
