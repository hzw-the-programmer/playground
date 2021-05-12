<?php
function sp_deviceStandbyHB($dbh) {
    $dbh->exec("
        DROP PROCEDURE IF EXISTS deviceStandbyHB;
    ");
    $dbh->exec("
        CREATE PROCEDURE deviceStandbyHB(
            sn VARCHAR(20),
            dt INT
        )
        sp: BEGIN
            DECLARE slot, port, type INT;

            DECLARE done INT DEFAULT FALSE;

            DECLARE c1 CURSOR FOR
            SELECT ci.slot, ci.port, ci.type
            FROM mpoint AS mp
            LEFT JOIN channels_info AS ci
            ON mp.ciid = ci.id
            LEFT JOIN devices_info AS di
            ON ci.device_id = di.id
            WHERE di.sn = sn AND
                  mp.endtime = 0;

            DECLARE CONTINUE HANDLER FOR NOT FOUND SET done = TRUE;

            OPEN c1;

            lp: LOOP
                FETCH c1 INTO slot, port, type;
                IF done THEN
                    LEAVE lp;
                END IF;
                CALL hbUpdateStatus(sn, slot, port, type, dt, dt, 0, 0x50, 0x50, 0, NULL);
            END LOOP;

            CLOSE c1;
        END;
    ");
}
