<?php
include_once "./zprotocal.php";
include_once "./device.php";
include_once "./channel.php";

class Db {
    private $dsn;
    private $username;
    private $passwd;

    public function __construct($dsn, $username, $passwd) {
        $this->dsn = $dsn;
        $this->username = $username;
        $this->passwd = $passwd;
    }

    public function newChannelsData($sn, $dt, $data) {
        $sql = "EXEC new_channel_data :sn, :slot, :port, :type, :ctime, :data";
        $dt = date("Y-m-d H:i:s", $dt);
        try {
            $dbh = new PDO($this->dsn, $this->username, $this->passwd);
            $stmt = $dbh->prepare($sql);
            $stmt->bindParam(":sn", $sn);
            $stmt->bindParam(":slot", $slot);
            $stmt->bindParam(":port", $port);
            $stmt->bindParam(":type", $type);
            $stmt->bindParam(":ctime", $dt);
            $stmt->bindParam(":data", $ad);
            for ($i = 0; $i < z_get_channels_data_nitem($data); $i++) {
                list($slot, $port, $type, $ad) = z_get_channels_data_item($data, $i);
                $ret = $stmt->execute();
                if (!$ret) {
                    print_r($stmt->errorInfo());
                }
            }
        } catch (PDOException $e) {
            echo $e->getMessage() . "\n";
        } finally {
            $stmt = NULL;
            $dbh = NULL;
        }
    }

    public function newChannelsStatus($sn, $dt, $data) {
        $sql = "EXEC new_channel_status :sn, :slot, :port, :type, :ctime, :status";
        $dt = date("Y-m-d H:i:s", $dt);
        try {
            $dbh = new PDO($this->dsn, $this->username, $this->passwd);
            $stmt = $dbh->prepare($sql);
            $stmt->bindParam(":sn", $sn);
            $stmt->bindParam(":slot", $slot);
            $stmt->bindParam(":port", $port);
            $stmt->bindParam(":type", $type);
            $stmt->bindParam(":ctime", $dt);
            $stmt->bindParam(":status", $status);
            for ($i = 0; $i < z_get_channels_status_nitem($data); $i++) {
                list($slot, $port, $type, $status) = z_get_channels_status_item($data, $i);
                $ret = $stmt->execute();
                if (!$ret) {
                    print_r($stmt->errorInfo());
                }
            }
        } catch (PDOException $e) {
            echo $e->getMessage() . "\n";
        } finally {
            $stmt = NULL;
            $dbh = NULL;
        }
    }

    public function getDevice(&$devices, $row) {
        $device = NULL;
        
        foreach ($devices as $dev) {
            if ($dev->sn == $row["sn"]) {
                $device = $dev;
                break;
            }
        }

        if ($device == NULL) {
            $device = new Device($row["sn"]);
            $device->id = $row["did"];
            $device->ip = $row["ip"];
            $device->port = $row["tport"];
            $device->deveui = $row["deveui"];
            $devices[] = $device;
        }

        return $device;
    }

    public function getChannel(&$channels, $row) {
        $channel = NULL;

        foreach ($channels as $chan) {
            if ($chan->id == $row["cid"]) {
                $channel = $chan;
                break;
            }
        }

        if ($channel == NULL) {
            $channel = new Channel();
            $channel->id = $row["cid"];
            $channel->slot = $row["slot"];
            $channel->port = $row["port"];
            $channel->type = $row["type"];
            $channel->ctime = $row["cctime"];
            $channel->cstatus = $row["cstatus"];
            $channel->ptime = $row["pctime"];
            $channel->pstatus = $row["pstatus"];

            $channels[] = $channel;
        }

        return $channel;
    }

    public function getDevices() {
        $sql = "
            SELECT
            d.id AS did, d.sn, d.ip, d.port AS tport, d.deveui,
            c.id AS cid, c.slot, c.port, c.type,
            rs.ctime AS cctime, rs.status AS cstatus,
            ps.ctime AS pctime, ps.status AS pstatus
            FROM realtime_status AS rs
            LEFT JOIN channels AS c
            ON rs.cid = c.id
            LEFT JOIN devices AS d
            ON c.did = d.id
            LEFT JOIN pre_status AS ps
            ON rs.cid = ps.cid AND rs.pid = ps.pid
            ORDER BY d.sn, c.slot, c.port, c.type";

        try {
            $dbh = new PDO($this->dsn, $this->username, $this->passwd);
            $stmt = $dbh->prepare($sql);
            $ret = $stmt->execute();
            if (!$ret) {
                print_r($stmt->errorInfo());
                return;
            }

            $devices = [];
            while ($row = $stmt->fetch(PDO::FETCH_ASSOC)) {
                $device = $this->getDevice($devices, $row);
                $channel = $this->getChannel($device->channels, $row);
            }
            
            return $devices;
        } catch (PDOException $e) {
            echo $e->getMessage() . "\n";
        } finally {
            $stmt = NULL;
            $dbh = NULL;
        }
    }
}
