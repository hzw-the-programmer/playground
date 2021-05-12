<?php
class DatabaseManager {
    const SQL = [
        'test' =>
            /*"WAITFOR DELAY '00:00:5';" . */'INSERT INTO test (num) VALUES (:num)',
    ];

    private $dsn;
    private $user;
    private $passwd;
    private $serv;

    private $dbh;
    private $stmts;

    function __construct($server, $user, $passwd, $serv) {
        $this->dsn = "sqlsrv:Server=$server;Database=test;LoginTimeout=1";
        $this->user = $user;
        $this->passwd = $passwd;
        $this->serv = $serv;

        $this->dbh = null;
        $this->stmts = null;
    }

    function getLogger() {
        return $this->serv;
    }

    function connect() {
        $logger = $this->getLogger();

        try {
            $this->dbh = new \PDO($this->dsn, $this->user, $this->passwd, [
                \PDO::ATTR_ERRMODE => \PDO::ERRMODE_EXCEPTION,
                \PDO::SQLSRV_ATTR_QUERY_TIMEOUT => 1
            ]);

            foreach(DatabaseManager::SQL as $name => $sql) {
                $this->stmts[$name] = $this->dbh->prepare($sql);
            }
        } catch (\PDOException $e) {
            $this->dbh = null;
            $this->stmts = null;

            $logger->critical(
                sprintf("connect failed: %s, %s, %s.",
                    $e->errorInfo[0], $e->errorInfo[1], $e->errorInfo[2]
                )
            );
        }
    }

    function transaction($name, $cb, $onFail = null, $onSucc = null) {
        $logger = $this->getLogger();
        $err = null;

        $dbh = $this->dbh;
        if (!$dbh) {
            $err = ['HZW00', 0, 'not connected'];
            goto end;
        }

        try {
            $dbh->beginTransaction();

            $cb($this->stmts[$name]);
            
            $dbh->commit();
        } catch (\PDOException $e) {
            if ($dbh->inTransaction()) {
                $dbh->rollBack();
            }

            $this->dbh = null;
            $this->stmts = null;

            $err = $e->errorInfo;
        }
    
        end:
        if ($err) {
            $logger->critical(sprintf(
                "transaction failed: %s, %s, %s.", $err[0], $err[1], $err[2]
            ));

            if ($onFail) $onFail($err);
        } else {
            if ($onSucc) $onSucc();
        }
    }
}
