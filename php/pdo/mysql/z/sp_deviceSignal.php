<?php
function sp_deviceSignal($dbh) {
    $dbh->exec("
        DROP PROCEDURE IF EXISTS deviceSignal;
    ");
    $dbh->exec("
        CREATE PROCEDURE deviceSignal(sn VARCHAR(20), rssi INT, dt INT)
        sp: BEGIN
            UPDATE device_rssi AS dr
            LEFT JOIN devices_info AS di
            ON dr.device_id = di.id
            SET dr.rssi = rssi,
                dr.updatetime = dt
            WHERE di.sn = sn;
        END;
    ");
}
