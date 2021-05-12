<?php
use Monolog\Logger;

class Config {
    const DIR = __DIR__;
    const LOG_DIR = Config::DIR . '/log';
    const LOG_LEVEL = Logger::DEBUG;
}
