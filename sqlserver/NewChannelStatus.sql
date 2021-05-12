SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[NewChannelStatus]
  @sn bigint,
  @slot smallint,
  @port smallint,
  @type smallint,
  @nctime datetime,
  @nstatus smallint
AS
BEGIN
  DECLARE @cid bigint
  DECLARE @pid bigint
  DECLARE @cctime datetime
  DECLARE @cstatus smallint

  SELECT @cid = rs.cid,
         @pid = rs.pid,
         @cctime = rs.ctime,
         @cstatus = rs.status
  FROM realtime_status AS rs
  LEFT JOIN channels AS c
  ON rs.cid = c.id
  LEFT JOIN devices AS d
  ON c.did = d.id
  WHERE d.sn = @sn AND
        c.slot = @slot AND
        c.port = @port AND
        c.type = @type

  IF @cid IS NULL
    RETURN 1

  DECLARE @pctime datetime
  DECLARE @pstatus smallint

  SELECT @pctime = ctime,
         @pstatus = status
  FROM pre_status
  WHERE cid = @cid AND
        pid = @pid

  IF @nstatus = 259 --ONLINE
  BEGIN
	UPDATE realtime_status
    SET ctime = @pctime,
        status = @pstatus
    WHERE cid = @cid AND
          pid = @pid AND
          status = 256 --OFFLINE
  END
  ELSE
  IF @nstatus != 260 AND --POWER ON
     (@nctime > @cctime OR @cstatus = 258) --ON
  BEGIN
    UPDATE realtime_status
    SET ctime = @nctime,
        status = @nstatus
    WHERE cid = @cid AND
          pid = @pid

    IF @nstatus != 256 --OFFLINE
    BEGIN
      UPDATE pre_status
      SET ctime = @nctime,
        status = @nstatus
      WHERE cid = @cid AND
          pid = @pid
    END
  END

  INSERT INTO channel_status
  (cid, pid, ctime, status)
  VALUES
  (@cid, @pid, @nctime, @nstatus)
END
