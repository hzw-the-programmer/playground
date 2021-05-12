<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

class DatabaseManager {
    const CONNECTED = 'DatabaseManager::CONNECTED';
    const DISCONNECTED = 'DatabaseManager::DISCONNECTED';
    const FAIL = 'DatabaseManager::FAIL';
    const SUCCESS = 'DatabaseManager::SUCCESS';

    private $dsn;
    private $user;
    private $passwd;
    private $logger;
    
    private $dbh;
    private $listeners = [];

    function __construct($dsn, $user, $passwd, $logger) {
        $this->dsn = $dsn;
        $this->user = $user;
        $this->passwd = $passwd;
        $this->logger = $logger;
    }

    function getDbh() {
        return $this->dbh;
    }

    function addListener($event, $listener) {
        $this->listeners[$event][] = $listener;
    }

    function dispatch($event) {
        foreach ($this->listeners[$event] as $listener) {
            $listener($this->dbh);
        }
    }

    function connect() {
        $logger = $this->logger;
        if ($this->dbh) return self::CONNECTED;

        try {
            $this->dbh = new \PDO($this->dsn, $this->user, $this->passwd, [
                \PDO::ATTR_ERRMODE => \PDO::ERRMODE_EXCEPTION,
            ]);

            $logger->critical('connect success.');

            $this->dispatch(self::CONNECTED);
        } catch (\PDOException $e) {
            $logger->critical(
                sprintf('connect fail: %s, %s, %s.',
                    $e->errorInfo[0], $e->errorInfo[1], $e->errorInfo[2]
                )
            );

            $this->dbh = null;
            $this->dispatch(static::DISCONNECTED);
        }

        return $this->dbh ? static::CONNECTED : static::DISCONNECTED;
    }

    function disconnect() {
        $this->dbh = null;
        $this->dispatch(static::DISCONNECTED);
    }

    function transaction($onTransaction) {
        $logger = $this->logger;

        if (!$this->dbh) {
            return static::FAIL;
        }

        try {
            $this->dbh->beginTransaction();

            $onTransaction($this->dbh);

            $this->dbh->commit();

            return static::SUCCESS;
        } catch (\PDOException $e) {
            $err = $e->errorInfo;
            $logger->critical(
                sprintf("transaction fail: %s, %s, %s.",
                    $err[0], $err[1], isset($err[2]) ? $err[2] : ''
                )
            );

            if ($this->dbh->inTransaction()) {
                try {
                    $this->dbh->rollBack();
                } catch (\PDOException $e) {
                    $err = $e->errorInfo;
                    $logger->critical(
                        sprintf("rallBack fail: %s, %s, %s.",
                            $err[0], $err[1], isset($err[2]) ? $err[2] : ''
                        )
                    );
                }
            }

            if (($err[0] === '42000' && $err[1] === 8114)
                || ($err[0] === '42000' && $err[1] === 8144)
                || ($err[0] === '42000' && $err[1] === 2812) // Could not find stored procedure
                || ($err[0] === 'HY093' && $err[1] === 0)) // Invalid parameter number
            {
                return static::FAIL;
            } else {
                $this->dispatch(static::DISCONNECTED);
                try {
                    $this->dbh = null;
                } catch (\PDOException $e) {
                    $err = $e->errorInfo;
                    $logger->critical(
                        sprintf("disconnect fail: %s, %s, %s.",
                            $err[0], $err[1], isset($err[2]) ? $err[2] : ''
                        )
                    );
                    $this->dbh = null;
                }
                return static::DISCONNECTED;
            }
        }
    }
}
