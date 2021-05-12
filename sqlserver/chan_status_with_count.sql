USE [iot]
GO

/****** Object:  View [dbo].[chan_status_with_count]    Script Date: 2018/6/8 19:56:03 ******/
SET ANSI_NULLS ON
GO

SET QUOTED_IDENTIFIER ON
GO

CREATE VIEW [dbo].[chan_status_with_count] AS
SELECT
id, ciid, pid, starttime, endtime, endtime - starttime AS count, status
FROM chan_status
GO

