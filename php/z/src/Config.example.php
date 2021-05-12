<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

use Monolog\Logger;

class Config {
    const DB_SERVER = '10.0.2.2,1433';
    const DB_USER = 'username';
    const DB_PW = 'password';
    const DB = 'dbname';
    const DB_CONNECT_INTERVAL = 10000;
    const DATA_RESTORE_INTERVAL = 10000;

    const LORA_SERVER = 'ip,port';
    const LORA_APPEUI = 'appeui';
    const LORA_HEARTBEAT_INTERVAL = 60000;
    const LORA_RECONNECT_INTERVAL = 10000;

    const DELAY = 10000;
    const OFFLINE_TIMEOUT = 100;

    const NAME = 'hzw';

    # sudo useradd --system --user-group iot
    const USER = 'iot';
    const GROUP = 'iot';

    const LOGGER_LEVEL = Logger::CRITICAL;

    const LOG_DIR = '/var/log/iot';
    const DATA_DIR = '/var/run/iot';

    const LOG_FILE_MAX_SIZE = 10 * 1024 * 1024;

    const WS_PORT = 9998;
    const UDP_PORT = 19268;

    const DSN = 'sqlsrv:Server=' . Config::DB_SERVER . ';Database=' . Config::DB . ';LoginTimeout=1';
}
