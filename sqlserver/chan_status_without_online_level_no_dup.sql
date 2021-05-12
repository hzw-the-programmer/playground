USE [iot]
GO

/****** Object:  View [dbo].[chan_status_without_online_level_no_dup]    Script Date: 2018/6/8 19:57:11 ******/
SET ANSI_NULLS ON
GO

SET QUOTED_IDENTIFIER ON
GO



CREATE VIEW [dbo].[chan_status_without_online_level_no_dup] AS
SELECT
id, ciid, pid, time, status
FROM (
	SELECT
	id, ciid, pid, time,
	LAG(status) OVER (PARTITION BY ciid, pid ORDER BY time) AS pstatus,
	status,
	LEAD(status) OVER (PARTITION BY ciid, pid ORDER BY time) AS nstatus
	FROM chan_status_without_online_level
) AS t
WHERE pstatus IS NULL OR pstatus != status


GO

