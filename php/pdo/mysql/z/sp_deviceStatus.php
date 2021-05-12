<?php
function sp_deviceStatus($dbh) {
    $dbh->exec("
        DROP PROCEDURE IF EXISTS deviceStatus;
    ");
    $dbh->exec("
        CREATE PROCEDURE deviceStatus (
            sn VARCHAR(20), dt INT, status INT
        )
        sp: BEGIN
            DECLARE done INT DEFAULT FALSE;

            DECLARE slot, port, type INT;

            DECLARE c1 CURSOR FOR
            SELECT ci.slot, ci.port, ci.type
            FROM mpoint AS mp
            LEFT JOIN channels_info AS ci
            ON mp.ciid = ci.id
            LEFT JOIN devices_info AS di
            ON ci.device_id = di.id
            WHERE di.sn = sn AND mp.endtime = 0;

            DECLARE CONTINUE HANDLER FOR NOT FOUND SET done = TRUE;

            OPEN c1;

            lp: LOOP
                FETCH c1 INTO slot, port, type;
                IF done THEN
                    LEAVE lp;
                END IF;
                CALL statusData(sn, slot, port, type, dt, dt, 0, status, status, 0, NULL);
            END LOOP;

            CLOSE c1;
        END;
    ");
}
