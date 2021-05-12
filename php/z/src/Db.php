<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

class Db {
    public $serv;

    public function __construct($serv) {
        $this->serv = $serv;
        $this->dsn = "sqlsrv:Server=" . Config::DB_SERVER . ";Database=" . Config::DB;
        $this->username = Config::DB_USER;
        $this->password = Config::DB_PW;
    }

    public function newChannelsData($sn, $seq, $dt, $pcdt, $body)
    {
        $logger = $this->serv->logger;

        $sql = "EXEC new_channel_data :sn, :slot, :port, :type, :time, :seq, :pctime, :data";

        try {
            $dbh = new \PDO($this->dsn, $this->username, $this->password);

            $stmt = $dbh->prepare($sql);

            $stmt->bindParam(":sn", $sn);
            $stmt->bindParam(":slot", $slot);
            $stmt->bindParam(":port", $port);
            $stmt->bindParam(":type", $type);
            $stmt->bindParam(":time", $dt);
            $stmt->bindParam(":seq", $seq);
            $stmt->bindParam(":pctime", $pcdt);
            $stmt->bindParam(":data", $data);

            foreach ($body->channels as $channel) {
                $slot = $channel->slot;
                $port = $channel->port;
                $type = $channel->type;
                $data = $channel->data;

                $logger->debug("Data:$sn,$slot,$port," . Channel::$types[$type] . ",$data," . date('Y-m-d H:i:s', $dt));

                $ret = $stmt->execute();
                if (!$ret) {
                    $logger->critical(print_r($stmt->errorInfo(), true));
                }
            }
        } catch (\PDOException $e) {
            $logger->critical($e->getMessage());
        } finally {
            $stmt = NULL;
            $dbh = NULL;
        }
    }

    public function newChannelsStatus($sn, $seq, $dt, $pcdt, $body)
    {
        $logger = $this->serv->logger;

        $sql = "EXEC new_channel_status :sn, :slot, :port, :type, :time, :seq, :pctime, :status";

        try {
            $dbh = new \PDO($this->dsn, $this->username, $this->password);

            $stmt = $dbh->prepare($sql);

            $stmt->bindParam(":sn", $sn);
            $stmt->bindParam(":slot", $slot);
            $stmt->bindParam(":port", $port);
            $stmt->bindParam(":type", $type);
            $stmt->bindParam(":time", $dt);
            $stmt->bindParam(":seq", $seq);
            $stmt->bindParam(":pctime", $pcdt);
            $stmt->bindParam(":status", $status);

            foreach ($body->channels as $channel) {
                $slot = $channel->slot;
                $port = $channel->port;
                $type = $channel->type;
                $status = $channel->status;

                $logger->debug("Status:$sn,$slot,$port," . Channel::$types[$type] . ",$status," . date('Y-m-d H:i:s', $dt));

                $ret = $stmt->execute();

                if (!$ret) {
                    $logger->critical(print_r($stmt->errorInfo(), true));
                }
            }
        } catch (\PDOException $e) {
            $logger->critical($e->getMessage());
        } finally {
            $stmt = NULL;
            $dbh = NULL;
        }
    }

    public function newDeviceStatus($sn, $pdt, $dt, $status) {
        $logger = $this->serv->logger;

        $sql = "EXEC new_device_status :sn, :dt, :status";

        try {
            $dbh = new \PDO($this->dsn, $this->username, $this->password);

            $stmt = $dbh->prepare($sql);

            $stmt->bindParam(":sn", $sn);
            $stmt->bindParam(":dt", $dt);
            $stmt->bindParam(":status", $status);

            $logger->debug(
                "DeviceStatus:$sn," . Device::$statuses[$status] .
                "," . date('Y-m-d H:i:s', $pdt) .
                "," . date('Y-m-d H:i:s', $dt)
            );

            $ret = $stmt->execute();
            if (!$ret) {
               $logger->critical(print_r($stmt->errorInfo(), true));
            }
        } catch (\PDOException $e) {
            $logger->critical($e->getMessage());
        } finally {
            $stmt = NULL;
            $dbh = NULL;
        }
    }

    public function newMpoints($sn, $channels, $dt) {
        $logger = $this->serv->logger;

        $sql = "EXEC new_mpoint :sn, :slot, :port, :type, :pid, :name, :dt";

        try {
            $dbh = new \PDO($this->dsn, $this->username, $this->password);

            $stmt = $dbh->prepare($sql);

            $stmt->bindParam(":sn", $sn);
            $stmt->bindParam(":slot", $slot);
            $stmt->bindParam(":port", $port);
            $stmt->bindParam(":type", $type);
            $stmt->bindParam(":pid", $pid);
            $stmt->bindParam(":name", $name);
            $stmt->bindParam(":dt", $dt);

            foreach ($channels as $channel) {
                $slot = $channel->slot;
                $port = $channel->port;
                $type = $channel->type;
                $pid = $channel->pid;
                $name = $channel->name;

                $logger->debug("NewMpoint:$sn,$slot,$port," . Channel::$types[$type] . ",$pid,$name," . date('Y-m-d H:i:s', $dt));

                $ret = $stmt->execute();
                if (!$ret) {
                    $logger->critical(print_r($stmt->errorInfo(), true));
                }
            }
        } catch (\PDOException $e) {
            $logger->critical($e->getMessage());
        } finally {
            $stmt = NULL;
            $dbh = NULL;
        }
    }
}
