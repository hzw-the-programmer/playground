<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
require __DIR__ . '/../vendor/autoload.php';
use Hzw\Task\LoadDevices;

$t = new LoadDevices();
$t->process();
