<?php
require __DIR__ . "/vendor/autoload.php";

use Monolog\Logger;
use Monolog\Handler\StreamHandler;

$log = new Logger('monolog_test');
$log->pushHandler(new StreamHandler('/tmp/monolog_test/1.log', Logger::WARNING));

$log->warning('FOO');
$log->error('Bar');
