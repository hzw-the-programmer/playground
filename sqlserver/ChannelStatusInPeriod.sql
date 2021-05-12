SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[ChannelStatusInPeriod]
	@pid bigint,
	@st datetime,
	@et datetime
AS
BEGIN
	SET NOCOUNT ON;

	DECLARE @t1 TABLE (
		id bigint,
		cid bigint,
		pid bigint,
		ctime datetime,
		status smallint
	)

	DECLARE @t2 TABLE (
		id bigint,
		cid bigint,
		pid bigint,
		ctime datetime,
		status smallint
	)

	DECLARE @t3 TABLE (
		id bigint,
		cid bigint,
		pid bigint,
		ctime datetime,
		status smallint
	)

	INSERT INTO @t1
	SELECT *
	FROM channel_status
	WHERE pid = @pid AND
		  ctime BETWEEN @st AND @et

	DECLARE @_st datetime, @_et datetime

	SELECT @_st = MAX(ctime)
	FROM channel_status
	WHERE pid = @pid AND
	      ctime <= @st

	SELECT @_et = MIN(ctime)
	FROM channel_status
	WHERE pid = @pid AND
	      ctime >= @et

	IF @_st IS NULL
	BEGIN
		noop1:
	END
	ELSE
	IF @_st != @st
	BEGIN
		INSERT INTO @t2 (id, cid, pid, ctime, status)
		SELECT id, cid, pid, @st, status
		FROM channel_status
		WHERE pid = @pid AND
		      ctime = @_st
	END

	IF @_et IS NULL
	BEGIN
		noop2:
	END
	ELSE
	IF @_et != @et
	BEGIN
		INSERT INTO @t2 (id, cid, pid, ctime, status)
		SELECT id, cid, pid, @et, status
		FROM channel_status
		WHERE pid = @pid AND
		      ctime = @_et
	END

	SELECT *
	FROM @t1
	UNION
    SELECT *
	FROM @t2
	UNION
	SELECT *
	FROM @t3
	ORDER BY ctime
END
