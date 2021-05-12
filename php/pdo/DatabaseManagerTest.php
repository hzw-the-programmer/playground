<?php
/*
connect failed: HYT00, 0, [unixODBC][Microsoft][ODBC Driver 17 for SQL Server]Login timeout expired.
transaction failed: HYT00, 0, [Microsoft][ODBC Driver 17 for SQL Server]Query timeout expired.
transaction failed: 42000, 8114, [Microsoft][ODBC Driver 17 for SQL Server][SQL Server]从数据类型 nvarchar 转换为 smallint 时出错。
transaction failed: 42000, 8144, [Microsoft][ODBC Driver 17 for SQL Server][SQL Server]为过程或函数 deviceSignal 指定了过多的参数。

transaction failed: 08S01, 10004, [Microsoft][ODBC Driver 17 for SQL Server]TCP Provider: Error code 0x2714.
connect failed: 08S01, 10004, [unixODBC][Microsoft][ODBC Driver 17 for SQL Server]TCP Provider: Error code 0x2714.
connect failed: 08001, 10004, [unixODBC][Microsoft][ODBC Driver 17 for SQL Server]TCP Provider: Error code 0x2714.

transaction failed: 08S01, 10060, [Microsoft][ODBC Driver 17 for SQL Server]TCP Provider: Error code 0x274C.

08S01, 10060, [Microsoft][ODBC Driver 17 for SQL Server]TCP Provider: Error code 0x274C.
08S01, 0, [Microsoft][ODBC Driver 17 for SQL Server]Communication link failure.

transaction failed: 08S01, 10060, [Microsoft][ODBC Driver 17 for SQL Server]TCP Provider: Error code 0x274C.
transaction failed: 08S01, -2147467259, [Microsoft][ODBC Driver 17 for SQL Server]Communication link failure.

transaction failed: 08S01, 10060, [Microsoft][ODBC Driver 17 for SQL Server]TCP Provider: Error code 0x274C.
query failed: 08S02, 32, [Microsoft][ODBC Driver 17 for SQL Server]TCP Provider: Error code 0x20.

transaction failed: 08S01, -2147467259, [Microsoft][ODBC Driver 17 for SQL Server]Communication link failure.
query failed: 08S02, 32, [Microsoft][ODBC Driver 17 for SQL Server]TCP Provider: Error code 0x20.
*/
require './vendor/autoload.php';

use Monolog\Logger;
use Monolog\Handler\StreamHandler;

if ($argc !== 4) {
    die("{$argv[0]} server user passwd\n");
}

$server = $argv[1];
$user = $argv[2];
$passwd = $argv[3];

const LOG_CHANNEL = 'databaseMgr';
$LOG_FILE = date('Y-m-d');
const LOG_LEVEL = Config::LOG_LEVEL;

if (!is_dir(Config::LOG_DIR)) {
    mkdir(Config::LOG_DIR);
}

$handler = new StreamHandler(Config::LOG_DIR . "/$LOG_FILE", LOG_LEVEL);
$logger = new Logger(LOG_CHANNEL);
$logger->pushHandler($handler);

$databaseMgr = new DatabaseManager($server, $user, $passwd, $logger);
$cost = measure([$databaseMgr, 'connect']);
$logger->debug("connect cost: $cost");

$cost = measure(function() use ($databaseMgr, $logger) {
    for ($i = 0; $i < 2; $i++) {
        for ($j = 0; $j < 10; $j++) {
            $cost = measure([$databaseMgr, 'transaction'], [
                'test',
                function($stmt) use ($j) {
                    $stmt->bindParam(':num', $j);
                    $stmt->execute();
                },
                function($err) use ($i, $j, $logger) {
                    $logger->critical("$i, $j fail.");
                },
                function () use ($i, $j, $logger) {
                    $logger->debug("$i, $j succ.");
                }
            ]);
            $logger->debug("transaction cost: $cost");
        }
    }
});
$logger->debug(sprintf("cost: $cost"));

function measure($func, $params = []) {
    $start = microtime(true);
    call_user_func_array($func, $params);
    return microtime(true) - $start;
}
