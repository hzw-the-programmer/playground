<?php
function sp_hbUpdateAvgData($dbh) {
    $dbh->exec("
        DROP PROCEDURE IF EXISTS hbUpdateAvgData;
    ");
    $dbh->exec("
        CREATE PROCEDURE hbUpdateAvgData(
            sn VARCHAR(20), slot INT, port INT, type INT,
            dt INT, pcdt INT, seq INT, avgData FLOAT
        )
        sp: BEGIN
            DECLARE mpid INT;

            SELECT mp.id INTO mpid
            FROM mpoint AS mp
            LEFT JOIN channels_info AS ci
            ON mp.ciid = ci.id
            LEFT JOIN devices_info AS di
            ON ci.device_id = di.id
            WHERE di.sn = sn AND
                  ci.slot = slot AND
                  ci.port = port AND
                  ci.type = type AND
                  mp.endtime = 0;

            IF mpid IS NULL THEN
                LEAVE sp;
            END IF;

            INSERT INTO mpoint_data
            (mpoint_id, swiftnum, data, time, createtime)
            VALUES
            (mpid, seq, avgData, dt, pcdt);
        END;
    ");
}
