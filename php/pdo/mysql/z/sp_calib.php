<?php
function sp_calib($dbh) {
    $dbh->exec("
        DROP PROCEDURE IF EXISTS calib;
    ");
    $dbh->exec("
        CREATE PROCEDURE calib(
            sn VARCHAR(20), slot INT, port INT, type INT,
            dt INT, pcdt INT, seq INT, result INT, data FLOAT
        )
        sp: BEGIN
            DECLARE did INT;
            DECLARE cid INT;

            SELECT di.id INTO did
            FROM devices_info AS di
            WHERE di.sn = sn;

            IF did IS NULL THEN
                LEAVE sp;
            END IF;

            SELECT ci.id INTO cid
            FROM channels_info AS ci
            WHERE ci.device_id = did AND
                  ci.slot = slot AND
                  ci.port = port AND
                  ci.type = type;

            IF cid IS NULL THEN
                INSERT INTO channels_info
                (device_id, slot, port, type, createtime, updatetime)
                VALUES
                (did, slot, port, type, pcdt, pcdt);

                SET cid = LAST_INSERT_ID();
            END IF;

            INSERT INTO calibration
            (cid, time, result, data)
            VALUES
            (cid, dt, result, data);
        END;
    ");
}
