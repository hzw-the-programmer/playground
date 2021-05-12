<?php
require __DIR__ . '/vendor/autoload.php';

use React\EventLoop\Factory;
use React\Filesystem\Filesystem;

$loop = Factory::create();

$filesystem = Filesystem::create($loop);
$file = $filesystem->file('./fs1.php');
$file->getContents()->then(function($contents) {
    echo "$contents\n";
});

$loop->run();
