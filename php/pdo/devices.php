<?php
if ($argc !== 5) {
    die("{$argv[0]} server db user passwd\n");
}

$server = $argv[1];
$db = $argv[2];
$user = $argv[3];
$passwd = $argv[4];

$dsn = "sqlsrv:Server=$server,1433;Database=$db";

$devMgr = new DeviceManager();

$dbh = new PDO($dsn, $user, $passwd);
$stmt = $dbh->query("
    SELECT di.id AS deviceId, di.sn, di.ip,
        ci.id AS channelId, ci.slot, ci.port, ci.type,
        mp.id AS mpointId
    FROM mpoint AS mp
    LEFT JOIN channels_info AS ci
    ON mp.ciid = ci.id
    LEFT JOIN devices_info AS di
    ON ci.device_id = di.id
    WHERE endtime = 0
    ORDER BY di.sn, ci.slot, ci.port, ci.type
");
while ($row = $stmt->fetch(PDO::FETCH_ASSOC)) {
    $deviceId = $row['deviceId'];
    $sn = $row['sn'];
    $ip = $row['ip'];

    $device = $devMgr->getDevice($sn);
    if (!$device) {
        $device = new Device($sn, $ip, $deviceId);
        $devMgr->addDevice($device);
    }

    $channelId = $row['channelId'];
    $slot = $row['slot'];
    $port = $row['port'];
    $type = $row['type'];
    $mpointId = $row['mpointId'];

    $channel = $device->getChannel($slot, $port, $type);
    if (!$channel) {
        $channel = new Channel($slot, $port, $type, $channelId, $mpointId);
        $device->addChannel($channel);
    }
}

$devicesCount = 0;
$channelsCount = 0;
foreach ($devMgr->getDevices() as $device) {
    $devicesCount++;
    $channelsCount += count($device->getChannels());
}
echo "devicesCount: $devicesCount\n";
echo "channelsCount: $channelsCount\n";

class DeviceManager {
    private $devices = [];

    function getDevice($sn) {
        return isset($this->devices[$sn])
            ? $this->devices[$sn]
            : null;
    }

    function addDevice($device) {
        $this->devices[$device->getSn()] = $device;
    }

    function getDevices() {
        return $this->devices;
    }
}

class Device {
    private $sn;
    private $ip;

    private $id;

    private $channels = [];

    function __construct($sn, $ip = '', $id = -1) {
        $this->sn = $sn;
        $this->ip = $ip;

        $this->id = $id;
    }

    function getSn() {
        return $this->sn;
    }

    function getChannel($slot, $port, $type) {
        foreach ($this->channels as $channel) {
            if ($channel->getSlot() === $slot
                && $channel->getPort() === $port
                && $channel->getType() === $type)
            {
                return $channel;
            }
        }
        return null;
    }

    function addChannel($channel) {
        $this->channels[] = $channel;
    }

    function getChannels() {
        return $this->channels;
    }
}

class Channel {
    private $slot;
    private $port;
    private $type;

    private $id;
    private $mpointId;

    function __construct($slot, $port, $type, $id = -1, $mpointId = -1) {
        $this->slot = $slot;
        $this->port = $port;
        $this->type = $type;

        $this->id = $id;
        $this->mpointId = $mpointId;
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
