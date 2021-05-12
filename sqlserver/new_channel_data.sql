USE [iot]
GO

/****** Object:  StoredProcedure [dbo].[new_channel_data]    Script Date: 2018/6/8 19:59:31 ******/
SET ANSI_NULLS ON
GO

SET QUOTED_IDENTIFIER ON
GO

-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-06-01
-- Description:	new data
-- =============================================
CREATE PROCEDURE [dbo].[new_channel_data]
	@sn NVARCHAR(20),
	@slot tinyint,
	@port tinyint,
	@type tinyint,
	@time bigint,
	@data float
AS
BEGIN
	DECLARE @ciid int, @pid int

	SELECT @ciid = ciid, @pid = pid
	FROM mpoint AS mp
	LEFT JOIN channel_info AS ci
	ON mp.ciid = ci.id
	WHERE ci.sn = @sn AND ci.slot = @slot AND ci.port = @port AND ci.porttype = @type

	IF @ciid IS NULL
	BEGIN
		RETURN
	END

	DECLARE @count int
	SELECT @count = COUNT(*) FROM channel_ad WHERE ciid = @ciid AND pid = @pid AND time = @time
	IF @count != 0
	BEGIN
		RETURN
	END

	INSERT INTO channel_ad
	(ciid, pid, ad_value, time)
	VALUES
	(@ciid, @pid, @data, @time)
END

GO

