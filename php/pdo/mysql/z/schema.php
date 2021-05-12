<?php
require '../config.php';
require 'tb_devices_info.php';
require 'tb_channels_info.php';
require 'tb_place.php';
require 'tb_mpoint.php';
require 'tb_mpoint_status.php';
require 'tb_mpoint_realtime_status.php';
require 'tb_calibration.php';
require 'tb_device_rssi.php';
require 'tb_mpoint_data.php';

require 'sp_openMpoint.php';
require 'sp_statusData.php';
require 'sp_deviceStatus.php';
require 'sp_offlineDevices.php';
require 'sp_calib.php';
require 'sp_closeMpoint.php';
require 'sp_deviceSignal.php';
require 'sp_updateDevice.php';
require 'sp_hbUpdateStatus.php';
require 'sp_hbUpdateAvgData.php';
require 'sp_deviceStandbyHB.php';

try {
    //$dbh = new PDO($dsn . ';charset=utf8mb4', $username, $passwd, $options);
    $dbh = new PDO($dsn, $username, $passwd, $options);

    /*
    $stmt = $dbh->query("SELECT @@CHARACTER_SET_CLIENT, @@CHARACTER_SET_RESULTS, @@COLLATION_CONNECTION");
    $rows = $stmt->fetch(PDO::FETCH_ASSOC);
    var_dump($rows);
    */

    tb_devices_info($dbh);
    tb_channels_info($dbh);
    tb_place($dbh);
    tb_mpoint($dbh);
    tb_mpoint_status($dbh);
    tb_mpoint_realtime_status($dbh);
    tb_calibration($dbh);
    tb_device_rssi($dbh);
    tb_mpoint_data($dbh);

    sp_openMpoint($dbh);
    sp_statusData($dbh);
    sp_deviceStatus($dbh);
    sp_offlineDevices($dbh);
    sp_calib($dbh);
    sp_closeMpoint($dbh);
    sp_deviceSignal($dbh);
    sp_updateDevice($dbh);
    sp_hbUpdateStatus($dbh);
    sp_hbUpdateAvgData($dbh);
    sp_deviceStandbyHB($dbh);
} catch (PDOException $e) {
    echo $e->getMessage() . PHP_EOL;
}
