USE [iot]
GO
/****** Object:  View [dbo].[chan_status_without_online]    Script Date: 2018/6/10 16:00:42 ******/
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
	sn, slot, port, porttype,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, ciid, pid, time, status
FROM (
	SELECT
		p0.name AS plant,
		p1.name AS workshop,
		p2.name AS region,
		p3.name AS line,
		p4.name AS station,
		mp.name AS mpoint,
		ci.sn, ci.slot, ci.port, ci.porttype,
		p0.id AS plantid,
		p1.id AS workshopid,
		p2.id AS regionid,
		p3.id AS lineid,
		p4.id AS stationid,
		cs.id, cs.ciid, cs.pid, time,
		LAG(status) OVER (PARTITION BY cs.ciid, cs.pid ORDER BY time) AS pstatus,
		status,
		LEAD(status) OVER (PARTITION BY cs.ciid, cs.pid ORDER BY time) AS nstatus
	FROM channel_status AS cs
	LEFT JOIN mpoint AS mp
	ON cs.ciid = mp.ciid AND cs.pid = mp.pid
	LEFT JOIN channel_info AS ci
	ON mp.ciid = ci.id
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
/****** Object:  View [dbo].[chan_status_without_online_level]    Script Date: 2018/6/10 16:00:42 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO


CREATE VIEW [dbo].[chan_status_without_online_level] AS
SElECT
	plant,
	workshop,
	region,
	line,
	station,
	mpoint,
	sn, slot, port, porttype,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, ciid, pid, time,
CASE
	WHEN status > 255 THEN status
	ELSE
	CASE
		WHEN porttype = 9 THEN
		CASE
			WHEN status & 0x80 = 0x80 THEN 0x80
			WHEN status & 0x50 = 0x50 THEN 0x50
			WHEN status & 0x40 = 0x40 THEN 0x40
			WHEN status & 0x20 = 0x20 THEN 0x20
			ELSE 0x00
		END
		ELSE
		CASE
			WHEN status & 0x20 = 0x20 THEN 0x20
			ELSE 0x00
		END
	END
END AS status
FROM chan_status_without_online


GO
/****** Object:  View [dbo].[chan_status_without_online_level_no_dup]    Script Date: 2018/6/10 16:00:42 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO



CREATE VIEW [dbo].[chan_status_without_online_level_no_dup] AS
SELECT
plant,
	workshop,
	region,
	line,
	station,
	mpoint,
	sn, slot, port, porttype,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, ciid, pid, time, status
FROM (
	SELECT
	plant,
	workshop,
	region,
	line,
	station,
	mpoint,
	sn, slot, port, porttype,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, ciid, pid, time,
	LAG(status) OVER (PARTITION BY ciid, pid ORDER BY time) AS pstatus,
	status,
	LEAD(status) OVER (PARTITION BY ciid, pid ORDER BY time) AS nstatus
	FROM chan_status_without_online_level
) AS t
WHERE pstatus IS NULL OR pstatus != status



GO
/****** Object:  View [dbo].[chan_status]    Script Date: 2018/6/10 16:00:42 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO


CREATE VIEW [dbo].[chan_status] AS
SELECT
	plant,
	workshop,
	region,
	line,
	station,
	mpoint,
	sn, slot, port, porttype,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, ciid, pid,
	time AS starttime,
	LEAD(time) OVER (PARTITION BY ciid, pid ORDER BY time) AS endtime,
	status
FROM chan_status_without_online_level_no_dup


GO
