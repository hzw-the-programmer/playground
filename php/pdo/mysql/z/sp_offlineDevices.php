<?php
function sp_offlineDevices($dbh) {
    $dbh->exec("
        DROP PROCEDURE IF EXISTS offlineDevices;
    ");
    $dbh->exec("
        CREATE PROCEDURE offlineDevices(dt INT)
        sp: BEGIN
            DECLARE offline INT DEFAULT 256;

            DECLARE done INT DEFAULT FALSE;

            DECLARE sn VARCHAR(20);

            DECLARE c1 CURSOR FOR
            SELECT di.sn FROM devices_info AS di;

            DECLARE CONTINUE HANDLER FOR NOT FOUND SET done = TRUE;

            OPEN c1;

            lp: LOOP
                FETCH c1 INTO sn;
                IF done THEN
                    LEAVE lp;
                END IF;
                CALL deviceStatus(sn, dt, offline);
            END LOOP;

            CLOSE c1;
        END;
    ");
}
