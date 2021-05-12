USE [kaifaiot_dev_gj]
GO

/****** Object:  View [dbo].[chan_status]    Script Date: 2018/8/11 19:17:18 ******/
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
	sn, slot, port, type AS porttype,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, ciid, pid,
	time AS starttime,
	LEAD(time) OVER (PARTITION BY mpoint_id ORDER BY time) AS endtime,
	status
FROM chan_status_without_online_no_dup








GO

