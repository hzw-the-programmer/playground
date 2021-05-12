USE [iot]
GO

/****** Object:  StoredProcedure [dbo].[new_mpoint]    Script Date: 2018/6/5 18:15:54 ******/
SET ANSI_NULLS ON
GO

SET QUOTED_IDENTIFIER ON
GO

-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-06-05
-- Description:	new_mpoint
-- =============================================
CREATE PROCEDURE [dbo].[new_mpoint]
	@sn NVARCHAR(20),
	@slot tinyint,
	@port tinyint,
	@type tinyint,
	@pid int,
	@name NVARCHAR(20),
	@dt bigint
AS
BEGIN
	DECLARE @ciid int

	SELECT @ciid = id
	FROM channel_info
	WHERE sn = @sn AND slot = @slot AND port = @port AND porttype = @type

	IF @ciid IS NULL
	BEGIN
		INSERT INTO channel_info
		(sn, slot, port, porttype, createtime, updatetime)
		VALUES
		(@sn, @slot, @port, @type, @dt, @dt)

		SELECT @ciid = SCOPE_IDENTITY()
	END

	DECLARE @mid int

	SELECT @mid = id FROM mpoint WHERE ciid = @ciid AND pid = @pid AND name = @name

	IF @mid IS NULL
	BEGIN
		INSERT INTO mpoint
		(ciid, pid, name, createtime, updatetime)
		VALUES
		(@ciid, @pid, @name, @dt, @dt)

		SELECT @mid = SCOPE_IDENTITY()
	END

	EXEC new_channel_status @sn, @slot, @port, @type, @dt, 258
END

GO

