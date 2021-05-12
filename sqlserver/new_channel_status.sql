USE [iot]
GO

/****** Object:  StoredProcedure [dbo].[new_channel_status]    Script Date: 2018/6/13 10:51:31 ******/
SET ANSI_NULLS ON
GO

SET QUOTED_IDENTIFIER ON
GO

-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-06-01
-- Description:	new_status
-- =============================================
CREATE PROCEDURE [dbo].[new_channel_status]
	@sn nvarchar(20),
	@slot tinyint,
	@port tinyint,
	@type tinyint,
	@ntime bigint,
	@nstatus smallint
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

	DECLARE @rtsid int
	SELECT @rtsid = id FROM realtime_status WHERE ciid = @ciid AND pid = @pid
	IF @rtsid IS NULL
	BEGIN
		INSERT INTO realtime_status
		(ciid, pid, status, time)
		VALUES
		(@ciid, @pid, @nstatus, @ntime)
		SELECT @rtsid = SCOPE_IDENTITY()
	END

	DECLARE @psid int
	SELECT @psid = id FROM pre_status WHERE ciid = @ciid AND pid = @pid
	IF @psid IS NULL
	BEGIN
		INSERT INTO pre_status
		(ciid, pid, status, time)
		VALUES
		(@ciid, @pid, @nstatus, @ntime)
		SELECT @psid = SCOPE_IDENTITY()
	END

	DECLARE @pstatus int, @ptime bigint
	SELECT @pstatus = status, @ptime = time
	FROM pre_status
	WHERE id = @psid

	DECLARE @cstatus int, @ctime bigint
	SELECT @cstatus = status, @ctime = time
	FROM realtime_status
	WHERE id = @rtsid

	IF @cstatus = 258 --ON
	BEGIN
		SELECT @ntime = @ctime + 1
	END
	ELSE
	BEGIN
		IF @nstatus = 260 --POWERON
		BEGIN
			SELECT @ntime = @ntime - 1
		END
	END

	IF @nstatus = 259 --ONLINE
	BEGIN
		UPDATE realtime_status
		SET status = @pstatus,
		    time = @ptime
		WHERE id = @rtsid AND status = 256
	END
	ELSE
	BEGIN
		IF @ntime > @ctime AND @nstatus != 260
		BEGIN
			UPDATE realtime_status
			SET status = @nstatus,
			    time = @ntime
			WHERE id = @rtsid
			
			IF @nstatus != 256
			BEGIN
				UPDATE pre_status
				SET status = @nstatus,
				    time = @ntime
				WHERE id = @psid
			END
		END
	END

	INSERT INTO channel_status
	(ciid, pid, status, time)
	VALUES
	(@ciid, @pid, @nstatus, @ntime)
END

GO

