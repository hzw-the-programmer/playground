USE [kaifaiot_dev_gj]
GO

/****** Object:  View [dbo].[chan_status_without_online_no_dup]    Script Date: 2018/8/11 19:17:03 ******/
SET ANSI_NULLS ON
GO

SET QUOTED_IDENTIFIER ON
GO






CREATE VIEW [dbo].[chan_status_without_online_no_dup] AS
SELECT
plant,
	workshop,
	region,
	line,
	station,
	mpoint,
	sn, slot, port, type,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, mpoint_id, ciid, pid, time, status
FROM (
	SELECT
	plant,
	workshop,
	region,
	line,
	station,
	mpoint,
	sn, slot, port, type,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, mpoint_id, ciid, pid, time,
	LAG(status) OVER (PARTITION BY mpoint_id ORDER BY time) AS pstatus,
	status,
	LEAD(status) OVER (PARTITION BY mpoint_id ORDER BY time) AS nstatus
	FROM chan_status_without_online
) AS t
WHERE pstatus IS NULL OR pstatus != status









GO

