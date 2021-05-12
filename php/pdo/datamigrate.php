<?php
if ($argc !== 9) {
    die("{$argv[0]} src_server src_db src_user src_passwd dist_server dist_db dist_user dist_passwd\n");
}

$srcServer = $argv[1];
$srcDb = $argv[2];
$srcUser = $argv[3];
$srcPasswd = $argv[4];

$distServer = $argv[5];
$distDb = $argv[6];
$distUser = $argv[7];
$distPasswd = $argv[8];

$srcDsn = "sqlsrv:Server=$srcServer,1433;Database=$srcDb";
$srcDbh = new PDO($srcDsn, $srcUser, $srcPasswd);

$distDsn = "sqlsrv:Server=$distServer,1433;Database=$distDb";
$distDbh = new PDO($distDsn, $distUser, $distPasswd);

$srcStmt = $srcDbh->query("
    SELECT di.sn, ci.slot, ci.port, ci.porttype AS type, mp.name,
        p5.name AS station, p4.name AS line, p3.name AS region, p2.name AS workshop, p1.name AS factory
    FROM mpoint AS mp
    LEFT JOIN channel_info AS ci
    ON mp.ciid = ci.id
    LEFT JOIN device_info AS di
    ON ci.sn = di.sn
    LEFT JOIN place AS p5
    ON mp.pid = p5.id
    LEFT JOIN place AS p4
    ON p5.pid = p4.id
    LEFT JOIN place AS p3
    ON p4.pid = p3.id
    LEFT JOIN place AS p2
    ON p3.pid = p2.id
    LEFT JOIN place AS p1
    ON p2.pid = p1.id
    ORDER BY di.sn, ci.slot, ci.port, ci.porttype
");

$newMpointSql = 'EXEC :mpointId = newMpoint :sn, :slot, :port, :type, :name, ' .
                ':factory, :workshop, :region, :line, :station';
$newMpointStmt = $distDbh->prepare($newMpointSql);
$newMpointStmt->bindParam(':mpointId', $mpointId, PDO::PARAM_INT, 1);
$newMpointStmt->bindParam(':sn', $sn);
$newMpointStmt->bindParam(':slot', $slot);
$newMpointStmt->bindParam(':port', $port);
$newMpointStmt->bindParam(':type', $type);
$newMpointStmt->bindParam(':name', $name);
$newMpointStmt->bindParam(':factory', $factory);
$newMpointStmt->bindParam(':workshop', $workshop);
$newMpointStmt->bindParam(':region', $region);
$newMpointStmt->bindParam(':line', $line);
$newMpointStmt->bindParam(':station', $station);

while ($row = $srcStmt->fetch(PDO::FETCH_ASSOC)) {
    $sn = $row['sn'];
    $slot = $row['slot'];
    $port = $row['port'];
    $type = $row['type'];
    $name = $row['name'];

    $factory = $row['factory'];
    $workshop = $row['workshop'];
    $region = $row['region'];
    $line = $row['line'];
    $station = $row['station'];

    if (!$newMpointStmt->execute() || $mpointId === 0) {
        var_dump($newMpointStmt->errorInfo(), $mpointId);
        die();
    }
}

/*
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-09-25
-- Description:	new station, return station id
-- =============================================
CREATE PROCEDURE newStation
	@factory NCHAR(50),
	@workshop NCHAR(50),
	@region NCHAR(50),
	@line NCHAR(50),
	@station NCHAR(50)
AS
BEGIN
	SET NOCOUNT ON

	DECLARE @factoryId INT = NULL
	SELECT @factoryId = id FROM place WHERE pid = 0 AND name = @factory

	IF @factoryId IS NULL
	BEGIN
		INSERT INTO place (pid, name, level, del) VALUES (0, @factory, 1, 0)
		SELECT @factoryId = SCOPE_IDENTITY()
	END

	DECLARE @workshopId INT = NULL
	SELECT @workshopId = id FROM place WHERE pid = @factoryId AND name = @workshop

	IF @workshopId IS NULL
	BEGIN
		INSERT INTO place (pid, name, level, del) VALUES (@factoryId, @workshop, 2, 0)
		SELECT @workshopId = SCOPE_IDENTITY()
	END

	DECLARE @regionId INT = NULL
	SELECT @regionId = id FROM place WHERE pid = @workshopId AND name = @region

	IF @regionId IS NULL
	BEGIN
		INSERT INTO place (pid, name, level, del) VALUES (@workshopId, @region, 3, 0)
		SELECT @regionId = SCOPE_IDENTITY()
	END

	DECLARE @lineId INT = NULL
	SELECT @lineId = id FROM place WHERE pid = @regionId AND name = @line

	IF @lineId IS NULL
	BEGIN
		INSERT INTO place (pid, name, level, del) VALUES (@regionId, @line, 4, 0)
		SELECT @lineId = SCOPE_IDENTITY()
	END

	DECLARE @stationId INT = NULL
	SELECT @stationId = id FROM place WHERE pid = @lineId AND name = @station

	IF @stationId IS NULL
	BEGIN
		INSERT INTO place (pid, name, level, del) VALUES (@lineId, @station, 5, 0)
		SELECT @stationId = SCOPE_IDENTITY()
	END

	RETURN @stationId
END
GO

SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-09-25
-- Description:	new channel, return channel id
-- =============================================
CREATE PROCEDURE newChannel
	@sn NCHAR(50),
	@slot INT,
	@port INT,
	@type INT
AS
BEGIN
	SET NOCOUNT ON;

	DECLARE @now BIGINT
	SELECT @now = DATEDIFF(SECOND, '1970-01-01 08:00:00', GETDATE())

	DECLARE @deviceId INT = NULL
	SELECT @deviceId = id FROM devices_info WHERE sn = @sn

	IF @deviceId IS NULL
	BEGIN
		INSERT INTO devices_info (parent_id, sn, ip, type, status, level, createtime, updatetime)
		VALUES (0, @sn, '', 'KEDAS', 0, 1, @now, @now)
		SELECT @deviceId = SCOPE_IDENTITY()
	END

	DECLARE @channelId INT = NULL
	SELECT @channelId = id FROM channels_info WHERE device_id = @deviceId AND slot = @slot AND port = @port AND type = @type

	IF @channelId IS NULL
	BEGIN
		INSERT INTO channels_info (device_id, slot, port, type, createtime, updatetime)
		VALUES (@deviceId, @slot, @port, @type, @now, @now)
		SELECT @channelId = SCOPE_IDENTITY()
	END

	RETURN @channelId
END
GO

SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-09-25
-- Description:	new mpoint, return mpoint id
-- =============================================
CREATE PROCEDURE newMpoint
	@sn NCHAR(50), @slot INT, @port INT, @type INT, @name NCHAR(50),
	@factory NCHAR(50), @workshop NCHAR(50), @region NCHAR(50), @line NCHAR(50), @station NCHAR(50)
AS
BEGIN
	SET NOCOUNT ON;

	DECLARE @now BIGINT
	SELECT @now = DATEDIFF(SECOND, '1970-01-01 08:00:00', GETDATE())

	DECLARE @channelId INT = NULL
	EXEC @channelId = newChannel @sn, @slot, @port, @type
	IF @channelId IS NULL
	BEGIN
		RETURN 0
	END

	DECLARE @stationId INT = NULL
	EXEC @stationId = newStation @factory, @workshop, @region, @line, @station
	IF @stationId IS NULL
	BEGIN
		RETURN 0
	END

	DECLARE @mpointId INT = NULL
	SELECT @mpointId = id FROM mpoint WHERE endtime = 0 AND ciid = @channelId AND pid = @stationId AND name = @name
	IF @mpointId IS NULL
	BEGIN
		INSERT INTO mpoint (pid, ciid, name, starttime, endtime)
		VALUES (@stationId, @channelId, @name, @now, 0)
		SELECT @mpointId = SCOPE_IDENTITY()
	END

	INSERT INTO mpoint_realtime_status (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
	VALUES (@mpointId, 0, 258, 258, 0, @now, @now)

	INSERT INTO mpoint_pre_status (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
	VALUES (@mpointId, 0, 258, 258, 0, @now, @now)

	INSERT INTO mpoint_status (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
	VALUES (@mpointId, 0, 258, 258, 0, @now, @now)

	RETURN @mpointId
END
GO
*/