<?php
require __DIR__ . "/vendor/autoload.php";

use Monolog\Logger;
use Monolog\Handler\StreamHandler;

$log = new Logger("zhiwenhe");
$log->pushHandler(new StreamHandler("./log", LOGGER::WARNING));

$log->warning('FOO');
$log->error('BAR');
