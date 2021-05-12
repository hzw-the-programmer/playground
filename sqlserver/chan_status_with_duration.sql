USE [kaifaiot]
GO

/****** Object:  View [dbo].[chan_status_with_duration]    Script Date: 2018/6/9 10:08:06 ******/
SET ANSI_NULLS ON
GO

SET QUOTED_IDENTIFIER ON
GO


CREATE VIEW [dbo].[chan_status_with_duration] AS
SELECT
id, ciid, pid, starttime, endtime, endtime - starttime AS duration, status
FROM chan_status

GO

