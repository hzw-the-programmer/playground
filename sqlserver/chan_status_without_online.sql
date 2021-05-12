USE [kaifaiot_dev_gj]
GO

/****** Object:  View [dbo].[chan_status_without_online]    Script Date: 2018/8/11 19:16:09 ******/
SET ANSI_NULLS ON
GO

SET QUOTED_IDENTIFIER ON
GO






CREATE VIEW [dbo].[chan_status_without_online] AS
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
		p0.name AS plant,
		p1.name AS workshop,
		p2.name AS region,
		p3.name AS line,
		p4.name AS station,
		mp.name AS mpoint,
		di.sn, ci.slot, ci.port, ci.type,
		p0.id AS plantid,
		p1.id AS workshopid,
		p2.id AS regionid,
		p3.id AS lineid,
		p4.id AS stationid,
		ms.id, ms.mpoint_id, mp.ciid, mp.pid, time,
		LAG(real_status) OVER (PARTITION BY ms.mpoint_id ORDER BY time) AS pstatus,
		real_status as status,
		LEAD(real_status) OVER (PARTITION BY ms.mpoint_id ORDER BY time) AS nstatus
	FROM mpoint_status AS ms
	LEFT JOIN mpoint AS mp
	ON ms.mpoint_id = mp.id
	LEFT JOIN channels_info AS ci
	ON mp.ciid = ci.id
	LEFT JOIN devices_info AS di
	ON ci.device_id = di.id
	LEFT JOIN place AS p4
	ON mp.pid = p4.id
	LEFT JOIN place AS p3
	ON p4.pid = p3.id
	LEFT JOIN place AS p2
	ON p3.pid = p2.id
	LEFT JOIN place AS p1
	ON p2.pid = p1.id
	LEFT JOIN place AS p0
	ON p1.pid = p0.id
) AS t
WHERE status != 259 AND (status != 256 OR (nstatus IS NULL OR nstatus = 260)) AND status != 260









GO

