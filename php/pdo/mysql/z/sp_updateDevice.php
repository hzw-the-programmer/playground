<?php
function sp_updateDevice($dbh) {
    $dbh->exec("
        DROP PROCEDURE IF EXISTS updateDevice;
    ");
    $dbh->exec("
        CREATE PROCEDURE updateDevice(
            sn VARCHAR(20), psn VARCHAR(20),
            ip VARCHAR(20), port INT, devaddr VARCHAR(20), dt INT
        )
        sp: BEGIN
            DECLARE pdid, did INT;
            DECLARE type VARCHAR(20);
            DECLARE level INT DEFAULT 1;

            IF psn = '' THEN
                SET pdid = 0;
            ELSE
                SELECT di.id INTO pdid
                FROM devices_info AS di
                WHERE di.sn = psn;
            END IF;

            IF pdid IS NULL THEN
                LEAVE sp;
            END IF;

            IF devaddr != '' THEN
                SET type = 'LORA';
            ELSE
                SET type = 'KEDAS';
            END IF;

            IF pdid != 0 THEN
                SET type = 'ZIGBEE';
                SET level = 2;
            END IF;

            SELECT di.id INTO did
            FROM devices_info AS di
            WHERE di.sn = sn;

            IF did IS NOT NULL THEN
                UPDATE devices_info AS di
                SET di.parent_id = pdid,
                    di.ip = ip,
                    di.port = port,
                    di.devaddr = devaddr,
                    di.updatetime = dt
                WHERE di.id = did;

                UPDATE devices_info AS di
                SET di.ip = ip,
                    di.port = port,
                    di.devaddr = devaddr,
                    di.updatetime = dt
                WHERE di.parent_id = did;
            ELSE
                INSERT INTO devices_info
                (parent_id, sn, ip, port, type, status, level, createtime, updatetime, devaddr)
                VALUES
                (pdid, sn, ip, port, type, 0, level, dt, dt, devaddr);

                SET did = LAST_INSERT_ID();

                INSERT INTO device_rssi
                (device_id, rssi, updatetime)
                VALUES
                (did, 99, dt);
            END IF;

            IF type != 'ZIGBEE' THEN
                UPDATE devices_info AS di
                SET di.ip = '', di.port = 0, di.updatetime = dt
                WHERE di.ip != '' AND di.ip = ip AND
                      di.port != 0 AND di.port = port AND
                      di.id != did AND
                      di.parent_id != did;

                UPDATE devices_info AS di
                SET di.devaddr = '', di.updatetime = dt
                WHERE di.devaddr != '' AND di.devaddr = devaddr AND
                      di.id != did AND
                      di.parent_id != did;
            END IF;
        END;
    ");
}
