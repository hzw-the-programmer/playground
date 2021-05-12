<?php
function sp_statusData($dbh) {
    $dbh->exec("
        DROP PROCEDURE IF EXISTS statusData;
    ");
    $dbh->exec("
        CREATE PROCEDURE statusData(
            sn VARCHAR(20),
            slot INT,
            port INT,
            type INT,
            dt INT,
            pcdt INT,
            seq INT,
            status INT,
            realStatus INT,
            level INT,
            data float
        )
        sp: BEGIN
            DECLARE poweroff INT DEFAULT 261;
            DECLARE poweron INT DEFAULT 260;
            DECLARE online INT DEFAULT 259;

            DECLARE mpid INT;
            DECLARE cdt INT;

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

            INSERT INTO mpoint_status
            (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime, data)
            VALUES
            (mpid, seq, status, realStatus, level, dt, pcdt, data);

            IF status = poweroff OR status = poweron OR status = online THEN
                LEAVE sp;
            END IF;

            SELECT time INTO cdt
            FROM mpoint_realtime_status
            WHERE mpoint_id = mpid;

            IF dt > cdt THEN
                UPDATE mpoint_realtime_status
                SET swiftnum = seq,
                    raw_status = status,
                    real_status = realStatus,
                    alarm_level = level,
                    time = dt,
                    createtime = pcdt,
                    data = data
                WHERE mpoint_id = mpid;
            END IF;
        END;
    ");
}
