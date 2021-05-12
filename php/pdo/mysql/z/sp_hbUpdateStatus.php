<?php
function sp_hbUpdateStatus($dbh) {
    $dbh->exec("
        DROP PROCEDURE IF EXISTS hbUpdateStatus;
    ");
    $dbh->exec("
        CREATE PROCEDURE hbUpdateStatus(
            sn VARCHAR(20), slot INT, port INT, type INT,
            dt INT, pcdt INT, seq INT, status INT, realStatus INT, level INT, data FLOAT
        )
        sp: BEGIN
            DECLARE mpid INT;
            DECLARE cdt, cstatus INT;

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

            SELECT mprs.time, mprs.raw_status INTO cdt, cstatus
            FROM mpoint_realtime_status AS mprs
            WHERE mprs.mpoint_id = mpid;

            IF dt > cdt AND status != cstatus THEN
                UPDATE mpoint_realtime_status AS mprs
                SET mprs.swiftnum = seq,
                    mprs.raw_status = status,
                    mprs.real_status = realStatus,
                    mprs.alarm_level = level,
                    mprs.time = dt,
                    mprs.createtime = pcdt,
                    mprs.data = data
                WHERE mprs.mpoint_id = mpid;

                INSERT INTO mpoint_status
                (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime, data)
                VALUES
                (mpid, seq, status, realStatus, level, dt, pcdt, data);
            END IF;
        END;
    ");
}
