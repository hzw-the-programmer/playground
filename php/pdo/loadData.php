<?php
if ($argc !== 5) {
    die("php {$argv[0]} server db user passwd\n");
}

$server = $argv[1];
$db = $argv[2];
$user = $argv[3];
$passwd = $argv[4];

$dsn = "sqlsrv:Server=$server;Database=$db";

try {
    $dbh = new PDO($dsn, $user, $passwd);
    $stmt = $dbh->prepare('
        SELECT
        di.id AS deviceId, di.sn, di.ip,
        ci.id AS channelId, ci.slot, ci.port, ci.type,
        mp.id AS mpointId,
        mprs.raw_status AS status, mprs.swiftnum AS seq, mprs.time AS dt,
        mpps.raw_status AS preStatus, mpps.swiftnum AS preSeq, mpps.time AS preDt
        FROM mpoint AS mp
        LEFT JOIN channels_info AS ci
        ON mp.ciid = ci.id
        LEFT JOIN devices_info AS di
        ON ci.device_id = di.id
        LEFT JOIN mpoint_realtime_status AS mprs
        ON mp.id = mprs.mpoint_id
        LEFT JOIN mpoint_pre_status AS mpps
        ON mp.id = mpps.mpoint_id
        WHERE mp.endtime = 0
    ');

    echo memory_get_usage() . PHP_EOL;

    if (!$stmt->execute()) {
        die(implode($stmt->errorInfo(), ',') . PHP_EOL);
    }

    echo memory_get_usage() . PHP_EOL;

    $devices = [];
    while ($row = $stmt->fetch(PDO::FETCH_ASSOC)) {
        $sn = $row['sn'];
        if (!isset($devices[$sn])) {
            $devices[$sn] = Device::fromDb($row);
        }
        $device = $devices[$sn];
        
        $slot = $row['slot'];
        $port = $row['port'];
        $type = $row['type'];
        if (!$device->getChannel($slot, $port, $type)) {
            $device->addChannel(Channel::fromDb($row));
        }
    }

    echo memory_get_usage() . PHP_EOL;
    $devices = [];
    echo memory_get_usage() . PHP_EOL;
    $stmt = null;
    echo memory_get_usage() . PHP_EOL;
    $dbh = null;
    echo memory_get_usage() . PHP_EOL;
} catch (PDOException $e) {
    die($e->getMessage() . PHP_EOL);
}

//var_dump($devices);

class Device {
    private $id;
    private $sn;
    private $ip;
    private $channels = [];

    function __construct($sn) {
        $this->sn = $sn;
    }

    static function fromDb($row) {
        $id = $row['deviceId'];
        $sn = $row['sn'];
        $ip = $row['ip'];

        $device = new Device($sn);
        $device->id = $id;
        $device->ip = $ip;

        return $device;
    }

    function getChannel($slot, $port, $type) {
        foreach ($this->channels as $channel) {
            if ($channel->getSlot() === $slot
                && $channel->getPort() === $port
                && $channel->getType() === $type
            ) {
                return $channel;
            }
        }
        return null;
    }

    function addChannel($channel) {
        $this->channels[] = $channel;
    }
}

class Channel {
    private $id;

    private $slot;
    private $port;
    private $type;

    private $mpointId;

    private $status;
    private $seq;
    private $dt;

    private $preStatus;
    private $preSeq;
    private $preDt;

    function __construct($slot, $port, $type = -1) {
        $this->slot = $slot;
        $this->port = $port;
        $this->type = $type;
    }

    static function fromDb($row) {
        $id = $row['channelId'];
        
        $slot = $row['slot'];
        $port = $row['port'];
        $type = $row['type'];

        $mpointId = $row['mpointId'];
        
        $status = $row['status'];
        $seq = $row['seq'];
        $dt = $row['dt'];

        $preStatus = $row['preStatus'];
        $preSeq = $row['preSeq'];
        $preDt = $row['preDt'];

        $channel = new Channel($slot, $port, $type);
        
        $channel->id = $id;
        
        $channel->mpointId = $mpointId;
        
        $channel->status = $status;
        $channel->seq = $seq;
        $channel->dt = $dt;

        $channel->preStatus = $preStatus;
        $channel->preSeq = $preSeq;
        $channel->preDt = $preDt;

        return $channel;
    }

    function getSlot() {
        return $this->slot;
    }

    function getPort() {
        return $this->port;
    }

    function getType() {
        return $this->type;
    }
}
