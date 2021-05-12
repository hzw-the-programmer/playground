USE [iot]
GO

/****** Object:  StoredProcedure [dbo].[new_device_status]    Script Date: 2018/6/2 13:44:04 ******/
SET ANSI_NULLS ON
GO

SET QUOTED_IDENTIFIER ON
GO

-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-06-02
-- Description:	online
-- =============================================
CREATE PROCEDURE [dbo].[new_device_status]
	@sn NVARCHAR(20),
	@dt bigint,
	@status smallint
AS
BEGIN
	DECLARE @slot tinyint, @port tinyint, @porttype tinyint

	DECLARE cc CURSOR FOR
	SELECT ci.slot, ci.port, ci.porttype
	FROM mpoint AS mp
	LEFT JOIN channel_info AS ci
	ON mp.ciid = ci.id
	WHERE ci.sn = @sn

	OPEN cc

	FETCH NEXT FROM cc
	INTO @slot, @port, @porttype

	WHILE @@FETCH_STATUS = 0
	BEGIN
		EXEC new_channel_status @sn, @slot, @port, @porttype, @dt, @status

		FETCH NEXT FROM cc
		INTO @slot, @port, @porttype
	END

	CLOSE cc

	DEALLOCATE cc
END

GO

