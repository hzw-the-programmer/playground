USE [iot]
GO

/****** Object:  View [dbo].[original_report]    Script Date: 2018/6/8 20:04:32 ******/
SET ANSI_NULLS ON
GO

SET QUOTED_IDENTIFIER ON
GO



CREATE VIEW [dbo].[original_report] AS
SELECT
p0.name AS plant,
p1.name AS workshop,
p2.name AS region,
p3.name AS line,
p4.name AS station,
mp.name AS mpoint,
ci.sn, ci.slot, ci.port, ci.porttype,
starttime, endtime, status, count,
p0.id AS plantid,
p1.id AS workshopid,
p2.id AS regionid,
p3.id AS lineid,
p4.id AS stationid
FROM chan_status_with_count AS cs
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


GO

