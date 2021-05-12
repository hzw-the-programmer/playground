<?php
function sp_openMpoint($dbh) {
    $dbh->exec("
        DROP PROCEDURE IF EXISTS openMpoint;
    ");
    $dbh->exec("
        CREATE PROCEDURE openMpoint(
            sn VARCHAR(20), slot INT, port INT, type INT,
            pid INT, name NVARCHAR(20), dt INT, OUT mpid INT
        )
        sp: BEGIN
            DECLARE mpOn INT DEFAULT 258;
            DECLARE mpOffline INT DEFAULT 256;

            DECLARE did INT;
            DECLARE cid INT;

            SELECT id INTO did
            FROM devices_info AS di
            WHERE di.sn = sn;

            IF did IS NULL THEN
                SET mpid = 0;
                LEAVE sp;
            END IF;

            SELECT id INTO cid
            FROM channels_info AS ci
            WHERE device_id = did AND
                  ci.slot = slot AND
                  ci.port = port AND
                  ci.type = type;

            IF cid IS NULL THEN
                INSERT INTO channels_info
                (device_id, slot, port, type, createtime, updatetime)
                VALUES
                (did, slot, port, type, dt, dt);

                SET cid = LAST_INSERT_ID();
            END IF;

            SELECT id INTO mpid
            FROM mpoint AS mp
            WHERE ciid = cid AND
                  mp.pid = pid AND
                  endtime = 0;

            IF mpid IS NULL THEN
                INSERT INTO mpoint
                (pid, ciid, name, starttime, endtime)
                VALUES
                (pid, cid, name, dt, 0);

                SET mpid = LAST_INSERT_ID();
            ELSE
                LEAVE sp;
            END IF;

            INSERT INTO mpoint_status
            (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
            VALUES
            (mpid, 0, mpOn, mpOn, 0, dt, dt);

            INSERT INTO mpoint_realtime_status
            (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
            VALUES
            (mpid, 0, mpOffline, mpOffline, 0, dt, dt);
        END;
    ");
}
