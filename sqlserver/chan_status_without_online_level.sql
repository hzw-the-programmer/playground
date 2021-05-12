USE [iot]
GO

/****** Object:  View [dbo].[chan_status_without_online_level]    Script Date: 2018/6/8 19:56:51 ******/
SET ANSI_NULLS ON
GO

SET QUOTED_IDENTIFIER ON
GO

CREATE VIEW [dbo].[chan_status_without_online_level] AS
SElECT
id, ciid, pid, time,
CASE
	WHEN status > 255 THEN status
	ELSE status & 0xF0
END AS status
FROM chan_status_without_online
GO

