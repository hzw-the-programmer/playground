<?php
function sp_closeMpoint($dbh) {
    $dbh->exec("
        DROP PROCEDURE IF EXISTS closeMpoint;
    ");
    $dbh->exec("
        CREATE PROCEDURE closeMpoint(mpid INT, dt INT)
        sp: BEGIN
            DECLARE off INT DEFAULT 257;

            INSERT INTO mpoint_status
            (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
            VALUES
            (mpid, 0, off, off, 0, dt, dt);

            DELETE FROM mpoint_realtime_status
            WHERE mpoint_id = mpid;

            UPDATE mpoint
            SET endtime = dt
            WHERE id = mpid AND endtime = 0;
        END;
    ");
}
