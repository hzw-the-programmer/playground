USE [master]
GO
/****** Object:  Database [hzwiot]    Script Date: 2018/7/27 16:06:40 ******/
CREATE DATABASE [hzwiot]
 CONTAINMENT = NONE
 ON  PRIMARY 
( NAME = N'hzwiot', FILENAME = N'D:\MSSQL\DATA\hzwiot.mdf' , SIZE = 5120KB , MAXSIZE = UNLIMITED, FILEGROWTH = 1024KB )
 LOG ON 
( NAME = N'hzwiot_log', FILENAME = N'D:\MSSQL\DATA\hzwiot_log.ldf' , SIZE = 123648KB , MAXSIZE = 2048GB , FILEGROWTH = 10%)
GO
ALTER DATABASE [hzwiot] SET COMPATIBILITY_LEVEL = 110
GO
IF (1 = FULLTEXTSERVICEPROPERTY('IsFullTextInstalled'))
begin
EXEC [hzwiot].[dbo].[sp_fulltext_database] @action = 'enable'
end
GO
ALTER DATABASE [hzwiot] SET ANSI_NULL_DEFAULT OFF 
GO
ALTER DATABASE [hzwiot] SET ANSI_NULLS OFF 
GO
ALTER DATABASE [hzwiot] SET ANSI_PADDING OFF 
GO
ALTER DATABASE [hzwiot] SET ANSI_WARNINGS OFF 
GO
ALTER DATABASE [hzwiot] SET ARITHABORT OFF 
GO
ALTER DATABASE [hzwiot] SET AUTO_CLOSE OFF 
GO
ALTER DATABASE [hzwiot] SET AUTO_CREATE_STATISTICS ON 
GO
ALTER DATABASE [hzwiot] SET AUTO_SHRINK OFF 
GO
ALTER DATABASE [hzwiot] SET AUTO_UPDATE_STATISTICS ON 
GO
ALTER DATABASE [hzwiot] SET CURSOR_CLOSE_ON_COMMIT OFF 
GO
ALTER DATABASE [hzwiot] SET CURSOR_DEFAULT  GLOBAL 
GO
ALTER DATABASE [hzwiot] SET CONCAT_NULL_YIELDS_NULL OFF 
GO
ALTER DATABASE [hzwiot] SET NUMERIC_ROUNDABORT OFF 
GO
ALTER DATABASE [hzwiot] SET QUOTED_IDENTIFIER OFF 
GO
ALTER DATABASE [hzwiot] SET RECURSIVE_TRIGGERS OFF 
GO
ALTER DATABASE [hzwiot] SET  DISABLE_BROKER 
GO
ALTER DATABASE [hzwiot] SET AUTO_UPDATE_STATISTICS_ASYNC OFF 
GO
ALTER DATABASE [hzwiot] SET DATE_CORRELATION_OPTIMIZATION OFF 
GO
ALTER DATABASE [hzwiot] SET TRUSTWORTHY OFF 
GO
ALTER DATABASE [hzwiot] SET ALLOW_SNAPSHOT_ISOLATION OFF 
GO
ALTER DATABASE [hzwiot] SET PARAMETERIZATION SIMPLE 
GO
ALTER DATABASE [hzwiot] SET READ_COMMITTED_SNAPSHOT OFF 
GO
ALTER DATABASE [hzwiot] SET HONOR_BROKER_PRIORITY OFF 
GO
ALTER DATABASE [hzwiot] SET RECOVERY FULL 
GO
ALTER DATABASE [hzwiot] SET  MULTI_USER 
GO
ALTER DATABASE [hzwiot] SET PAGE_VERIFY CHECKSUM  
GO
ALTER DATABASE [hzwiot] SET DB_CHAINING OFF 
GO
ALTER DATABASE [hzwiot] SET FILESTREAM( NON_TRANSACTED_ACCESS = OFF ) 
GO
ALTER DATABASE [hzwiot] SET TARGET_RECOVERY_TIME = 0 SECONDS 
GO
EXEC sys.sp_db_vardecimal_storage_format N'hzwiot', N'ON'
GO
USE [hzwiot]
GO
/****** Object:  StoredProcedure [dbo].[clear]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-07-11
-- Description:	clear
-- =============================================
CREATE PROCEDURE [dbo].[clear]
AS
BEGIN
	-- SET NOCOUNT ON added to prevent extra result sets from
	-- interfering with SELECT statements.
	SET NOCOUNT ON;

	DELETE FROM channels_info
	DELETE FROM device_param
	DELETE FROM devices_info
	DELETE FROM mpoint
	DELETE FROM mpoint_data
	DELETE FROM mpoint_pre_status
	DELETE FROM mpoint_realtime_status
	DELETE FROM mpoint_status
	DELETE FROM place

	DELETE FROM ws_worktime_daily
END


GO
/****** Object:  StoredProcedure [dbo].[delete_mpoint]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO

-- =============================================
-- Author:		wujun
-- Create date: 2018-06-26
-- Description:	根据 SN 删除监控点-》删除realtime_status->pre_status->mpoint表更新endtime
-- Update date: 2018-07-17
-- Description:	批量插入off状态
-- =============================================
CREATE PROCEDURE [dbo].[delete_mpoint]
 @sn NVARCHAR(100)

AS
BEGIN

	DECLARE @nowTime bigint

	SELECT @nowTime = (DATEDIFF(ss, '1970-01-01 08:00:00', GETDATE())) 

	DELETE mrs
	FROM mpoint_realtime_status AS mrs
	LEFT JOIN mpoint AS m
	ON mrs.mpoint_id = m.id
	LEFT JOIN channels_info AS ci
	ON m.ciid = ci.id
	LEFT JOIN devices_info AS di
	ON di.id = ci.device_id
	WHERE di.sn = @sn

	DELETE mps
	FROM mpoint_pre_status AS mps
	LEFT JOIN mpoint AS m
	ON mps.mpoint_id = m.id
	LEFT JOIN channels_info AS ci
	ON m.ciid = ci.id
	LEFT JOIN devices_info AS di
	ON di.id = ci.device_id
	WHERE di.sn = @sn
	
	EXEC insert_delete_status @sn, @nowTime, 257

    UPDATE mp
	SET endtime = @nowTime
	FROM mpoint as mp
	LEFT JOIN channels_info AS ci
	ON mp.ciid = ci.id
	LEFT JOIN devices_info AS di
	ON di.id = ci.device_id
	WHERE di.sn = @sn

END

GO
/****** Object:  StoredProcedure [dbo].[delete_mpoint_one]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO

-- =============================================
-- Author:		wujun
-- Create date: 2018-06-26
-- Description:	根据 MID 删除监控点-》删除realtime_status->pre_status->mpoint表更新endtime
-- =============================================
CREATE PROCEDURE [dbo].[delete_mpoint_one]
 @id int

AS
BEGIN

	DELETE mrs
	FROM mpoint_realtime_status AS mrs
	LEFT JOIN mpoint AS m
	ON mrs.mpoint_id = m.id
	WHERE m.id = @id

	DELETE mps
	FROM mpoint_pre_status AS mps
	LEFT JOIN mpoint AS m
	ON mps.mpoint_id = m.id
	WHERE m.id = @id

	UPDATE mpoint
	SET endtime = (DATEDIFF(ss, '1970-01-01 08:00:00', GETDATE()))
	WHERE id = @id

	INSERT INTO mpoint_status
           ([mpoint_id]
           ,[swiftnum]
           ,[raw_status]
           ,[real_status]
           ,[alarm_level]
           ,[time]
           ,[createtime])
     VALUES
           (
		   @id
           ,0
           ,257
           ,257
           ,0
           ,(DATEDIFF(ss, '1970-01-01 08:00:00', GETDATE()))
           ,(DATEDIFF(ss, '1970-01-01 08:00:00', GETDATE()))
		   )

END

GO
/****** Object:  StoredProcedure [dbo].[insert_delete_status]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
-- =============================================
-- Author:		WUJUN
-- Create date: 2018-07-17
-- Description:	根据SN批量插入OFF状态
-- =============================================
CREATE PROCEDURE [dbo].[insert_delete_status]
	@sn NVARCHAR(20),
	@dt BIGINT,
	@status SMALLINT
AS
BEGIN
	-- SET NOCOUNT ON added to prevent extra result sets from
	-- interfering with SELECT statements.
	SET NOCOUNT ON;

	DECLARE @mid BIGINT

	DECLARE cc CURSOR FOR
	SELECT mp.id
	FROM mpoint as mp
	LEFT JOIN channels_info AS ci
	ON mp.ciid = ci.id
	LEFT JOIN devices_info AS di
	ON di.id = ci.device_id
	WHERE di.sn = @sn

	OPEN cc

	FETCH NEXT FROM cc
	INTO @mid

	WHILE @@FETCH_STATUS = 0
	BEGIN

		INSERT INTO mpoint_status ([mpoint_id],[swiftnum] ,[raw_status] ,[real_status] ,[alarm_level] ,[time],[createtime])
		 VALUES(@mid, 0, @status, @status, 0, @dt, @dt)

		FETCH NEXT FROM cc
		INTO @mid
	END

	CLOSE cc

	DEALLOCATE cc
END

GO
/****** Object:  StoredProcedure [dbo].[new_channel_data]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-07-10
-- Description:	new_channel_data
-- =============================================
CREATE PROCEDURE [dbo].[new_channel_data]
	@sn NVARCHAR(20),
	@slot TINYINT,
	@port TINYINT,
	@type TINYINT,
	@time BIGINT,
	@seq INT,
	@pctime BIGINT,
	@data float
AS
BEGIN
	-- SET NOCOUNT ON added to prevent extra result sets from
	-- interfering with SELECT statements.
	SET NOCOUNT ON;

	DECLARE @mpid int

	SELECT @mpid = mp.id
	FROM mpoint AS mp
	LEFT JOIN channels_info AS ci
	ON mp.ciid = ci.id
	LEFT JOIN devices_info AS di
	ON ci.device_id = di.id
	WHERE di.sn = @sn AND
	      ci.slot = @slot AND
		  ci.port = @port AND
		  ci.type = @type AND
		  mp.endtime = 0

	IF @mpid IS NULL
	BEGIN
		RETURN
	END

	DECLARE @count int

	SELECT @count = COUNT(*)
	FROM mpoint_data
	WHERE mpoint_id = @mpid AND
	      time = @time AND
		  swiftnum = @seq

	EXEC updateOfflineTime @sn, @slot, @port, @type, @time

	IF @count = 0
	BEGIN
		INSERT INTO mpoint_data (mpoint_id, swiftnum, data, time, createtime)
		VALUES (@mpid, @seq, @data, @time, @pctime)
	END
END


GO
/****** Object:  StoredProcedure [dbo].[new_channel_status]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-07-10
-- Description:	new_channel_status
-- =============================================
CREATE PROCEDURE [dbo].[new_channel_status]
	@sn NVARCHAR(20),
	@slot TINYINT,
	@port TINYINT,
	@type TINYINT,
	@time BIGINT,
	@seq INT,
	@pctime BIGINT,
	@raw_status SMALLINT
AS
BEGIN
	-- SET NOCOUNT ON added to prevent extra result sets from
	-- interfering with SELECT statements.
	SET NOCOUNT ON;

	DECLARE @OFF SMALLINT
	DECLARE @ON SMALLINT
	DECLARE @OFFLINE SMALLINT
	DECLARE @ONLINE SMALLINT
	DECLARE @POWEROFF SMALLINT
	DECLARE @POWERON SMALLINT

	SET @OFF = 257
	SET @ON = 258
	SET @OFFLINE = 256
	SET @ONLINE = 259
	SET @POWEROFF = 261
	SET @POWERON = 260

	-----------------------------------------
	-- check mpoint exists
	-----------------------------------------
	DECLARE @mpid int

	SELECT @mpid = mp.id
	FROM mpoint AS mp
	LEFT JOIN channels_info AS ci
	ON mp.ciid = ci.id
	LEFT JOIN devices_info AS di
	ON ci.device_id = di.id
	WHERE di.sn = @sn AND
	      ci.slot = @slot AND
		  ci.port = @port AND
		  ci.type = @type AND
		  mp.endtime = 0

	IF @mpid IS NULL
	BEGIN
		RETURN
	END

	-----------------------------------------
	-- check status exists
	-----------------------------------------
	DECLARE @msid INT

	SELECT @msid = id
	FROM mpoint_status
	WHERE mpoint_id = @mpid AND
	      time = @time AND
		  swiftnum = @seq

	IF @msid IS NOT NULL
	BEGIN
		RETURN
	END

	-----------------------------------------
	-- process raw_status
	-----------------------------------------
	DECLARE @status SMALLINT, @level SMALLINT

	SELECT @status = CASE
		WHEN @raw_status < 256 THEN
		CASE
			WHEN @type = 9 THEN
			CASE
				WHEN @raw_status & 0x80 = 0x80 THEN 0x80
				WHEN @raw_status & 0x50 = 0x50 THEN 0x50
				WHEN @raw_status & 0x40 = 0x40 THEN 0x40
				WHEN @raw_status & 0x20 = 0x20 THEN 0x20
				ELSE 0x00
			END
			ELSE
			CASE
				WHEN @raw_status & 0x20 = 0x20 THEN 0x20
				ELSE 0x00
			END
		END
		ELSE @raw_status
	END

	SELECT @level = CASE
		WHEN @raw_status < 256 THEN @raw_status & 0x07
		ELSE 0
	END

	-----------------------------------------
	-- get current and pre status
	-----------------------------------------
	DECLARE @c_seq INT, @c_raw_status SMALLINT,
	        @c_status SMALLINT, @c_level SMALLINT,
			@c_time BIGINT, @c_createtime BIGINT

	SELECT @c_seq = swiftnum, @c_raw_status = raw_status,
	       @c_status = real_status, @c_level = alarm_level,
		   @c_time = time, @c_createtime = createtime
	FROM mpoint_realtime_status
	WHERE mpoint_id = @mpid

	DECLARE @p_seq INT, @p_raw_status SMALLINT,
	        @p_status SMALLINT, @p_level SMALLINT,
			@p_time BIGINT, @p_createtime BIGINT

	SELECT @p_seq = swiftnum, @p_raw_status = raw_status,
	       @p_status = real_status, @p_level = alarm_level,
		   @p_time = time, @p_createtime = createtime
	FROM mpoint_pre_status
	WHERE mpoint_id = @mpid

	IF @c_status = @ON
	BEGIN
		SELECT @time = @c_time + 1
	END
	ELSE
	BEGIN
		IF @status = @POWERON
		BEGIN
			SELECT @time = @time - 1
		END
	END

	-----------------------------------------
	-- insert into mpoint_status
	-----------------------------------------
	IF @status < @OFFLINE
	BEGIN
		EXEC updateOfflineTime @sn, @slot, @port, @type, @time
	END

	INSERT INTO mpoint_status (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
	VALUES (@mpid, @seq, @raw_status, @status, @level, @time, @pctime)

	IF @status = @POWERON
	BEGIN
		RETURN
	END

	-----------------------------------------
	-- process
	-----------------------------------------
	IF @status = @ONLINE
	BEGIN
		UPDATE mpoint_realtime_status
		SET swiftnum = @p_seq, raw_status = @p_raw_status,
		    real_status = @p_status, alarm_level = @p_level,
			time = @p_time, createtime = @p_createtime
		WHERE real_status = @OFFLINE
		  AND mpoint_id = @mpid
	END
	ELSE
	BEGIN
		IF @time > @c_time OR (@time = @c_time AND @seq > @c_seq)
		BEGIN
			UPDATE mpoint_realtime_status
			SET swiftnum = @seq, raw_status = @raw_status,
				real_status = @status, alarm_level = @level,
				time = @time, createtime = @pctime
			WHERE mpoint_id = @mpid

			IF @status != @OFFLINE
			BEGIN
				UPDATE mpoint_pre_status
				SET swiftnum = @seq, raw_status = @raw_status,
					real_status = @status, alarm_level = @level,
					time = @time, createtime = @pctime
				WHERE mpoint_id = @mpid
			END
		END
	END
END


GO
/****** Object:  StoredProcedure [dbo].[new_device_status]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-07-10
-- Description:	new_device_status
-- =============================================
CREATE PROCEDURE [dbo].[new_device_status]
	@sn NVARCHAR(20),
	@dt BIGINT,
	@status SMALLINT
AS
BEGIN
	-- SET NOCOUNT ON added to prevent extra result sets from
	-- interfering with SELECT statements.
	SET NOCOUNT ON;

	DECLARE @slot TINYINT, @port TINYINT, @type TINYINT

	DECLARE cc CURSOR FOR
	SELECT ci.slot, ci.port, ci.type
	FROM channels_info AS ci
	LEFT JOIN devices_info AS di
	ON ci.device_id = di.id
	WHERE di.sn = @sn

	OPEN cc

	FETCH NEXT FROM cc
	INTO @slot, @port, @type

	WHILE @@FETCH_STATUS = 0
	BEGIN
		EXEC new_channel_status @sn, @slot, @port, @type, @dt, 0, @dt, @status

		FETCH NEXT FROM cc
		INTO @slot, @port, @type
	END

	CLOSE cc

	DEALLOCATE cc
END


GO
/****** Object:  StoredProcedure [dbo].[new_mpoint]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-07-10
-- Description:	new_mpoint
-- =============================================
CREATE PROCEDURE [dbo].[new_mpoint]
	@sn NVARCHAR(20),
	@slot TINYINT,
	@port TINYINT,
	@type TINYINT,
	@pid INT,
	@name NVARCHAR(20),
	@dt bigint
AS
BEGIN
	-- SET NOCOUNT ON added to prevent extra result sets from
	-- interfering with SELECT statements.
	SET NOCOUNT ON;

	DECLARE @diid int

	SELECT @diid = id
	FROM devices_info
	WHERE sn = @sn

	IF @diid IS NULL
	BEGIN
		RETURN
	END

	DECLARE @ciid int
	
	SELECT @ciid = id
	FROM channels_info
	WHERE device_id = @diid AND
	      slot = @slot AND
		  port = @port AND
		  type = @type

	IF @ciid IS NULL
	BEGIN
		INSERT INTO channels_info (device_id, slot, port, type, createtime, updatetime)
		VALUES (@diid, @slot, @port, @type, @dt, @dt)

		SELECT @ciid = SCOPE_IDENTITY()
	END

	DECLARE @mpid INT

	SELECT @mpid = id
	FROM mpoint
	WHERE ciid = @ciid AND
	      pid = @pid AND
		  endtime = 0

	IF @mpid IS NULL
	BEGIN
		INSERT INTO mpoint (pid, ciid, name, starttime, endtime)
		VALUES (@pid, @ciid, @name, @dt, 0)

		SELECT @mpid = SCOPE_IDENTITY()
	END

	INSERT INTO mpoint_realtime_status (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
	VALUES (@mpid, 0, 258, 258, 0, @dt, @dt)

	INSERT INTO mpoint_pre_status (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
	VALUES (@mpid, 0, 258, 258, 0, @dt, @dt)

	INSERT INTO mpoint_status (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
	VALUES (@mpid, 0, 258, 258, 0, @dt, @dt)
END


GO
/****** Object:  StoredProcedure [dbo].[updateOfflineTime]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-07-27
-- Description:	updateOfflineTime
-- =============================================
CREATE PROCEDURE [dbo].[updateOfflineTime]
	@sn NVARCHAR(20),
	@slot TINYINT,
	@port TINYINT,
	@type TINYINT,
	@time BIGINT
AS
BEGIN
	-- SET NOCOUNT ON added to prevent extra result sets from
	-- interfering with SELECT statements.
	SET NOCOUNT ON;

	DECLARE @OFFLINE SMALLINT
	DECLARE @POWERON SMALLINT
	SET @OFFLINE = 256
	SET @POWERON = 260

	DECLARE @offlineTime BIGINT
	DECLARE @poweronTime BIGINT

	SELECT TOP 1 @offlineTime = mps.time
	FROM mpoint_status AS mps
	LEFT JOIN mpoint AS mp
	ON mps.mpoint_id = mp.id
	LEFT JOIN channels_info AS ci
	ON mp.ciid = ci.id
	LEFT JOIN devices_info AS di
	ON ci.device_id = di.id
	WHERE di.sn = @sn
	  AND ci.slot = @slot
	  AND ci.port = @port
	  AND ci.type = @type
	  AND mps.raw_status = @OFFLINE
	  AND mps.time < @time
	ORDER BY time DESC

	SELECT TOP 1 @poweronTime = mps.time
	FROM mpoint_status AS mps
	LEFT JOIN mpoint AS mp
	ON mps.mpoint_id = mp.id
	LEFT JOIN channels_info AS ci
	ON mp.ciid = ci.id
	LEFT JOIN devices_info AS di
	ON ci.device_id = di.id
	WHERE di.sn = @sn
	  AND ci.slot = @slot
	  AND ci.port = @port
	  AND ci.type = @type
	  AND mps.raw_status = @POWERON
	  AND mps.time > @time
	ORDER BY time

	IF @offlineTime IS NULL
	  OR @poweronTime IS NULL
	BEGIN
		RETURN
	END

	SET @time = @time + 100
	IF @time >= @poweronTime
	BEGIN
		SET @time = @poweronTime - 1
	END

	UPDATE mps
	SET mps.time = @time
	FROM mpoint_status AS mps
	LEFT JOIN mpoint AS mp
	ON mps.mpoint_id = mp.id
	LEFT JOIN channels_info AS ci
	ON mp.ciid = ci.id
	LEFT JOIN devices_info AS di
	ON ci.device_id = di.id
	WHERE di.sn = @sn
	  AND mps.raw_status = @OFFLINE
	  AND mps.time = @offlineTime
END


GO
/****** Object:  UserDefinedFunction [dbo].[fn_ConvertTime]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
/****** Script for SelectTopNRows command from SSMS  ******/


CREATE function [dbo].[fn_ConvertTime](@time as int, @interval as int, @unit as varchar(10), @starttime as int)
returns varchar(20)

as 
begin
	declare @ret varchar(20)

	declare @curweek int
	declare @ssweek int

	declare @a datetime
	declare @b datetime

	declare @curdate datetime
	declare @ssdate datetime

	SELECT @curdate = DATEADD(s, @time + 8 * 3600,'1970-01-01 00:00:00')
	SELECT @ssdate = DATEADD(s, @starttime + 8 * 3600,'1970-01-01 00:00:00')

	
	if(@unit = 'week')
	begin
	
		SELECT @curweek = DATEPART(wk,@curdate) 
		SELECT @ssweek = DATEPART(wk,@ssdate)

		if (@curweek-@ssweek < @interval)
			set @ret = @ssweek
		else
			set @ret = @curweek
	end
	else if(@unit = 'day')
	begin

		if (DATEDIFF(day,@ssdate, @curdate) < @interval)
			set @ret = convert(varchar(100),@ssdate,23)
		else
			set @ret = convert(varchar(100),@curdate,23)
	end

	return @ret
end




GO
/****** Object:  UserDefinedFunction [dbo].[fn_GetPlace]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO

/****** Script for SelectTopNRows command from SSMS  ******/


CREATE function [dbo].[fn_GetPlace](@id as int)
returns @tb table
(
	id int not null,
	pid int not null,
	name nvarchar(50),
	level int not null,
	del int not null
)

as
begin

with cte_child(id, pid, name, level, del)
as
(
	select id, pid, name ,level, del from place where id = @id

	union all

	select a.id, a.pid, a.name, a.level, a.del from  place a
	inner join
	cte_child b
	on a.pid = b.id
)

insert into @tb select * from cte_child;
return
end




GO
/****** Object:  Table [dbo].[channels_info]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[channels_info](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[device_id] [int] NOT NULL,
	[slot] [int] NOT NULL,
	[port] [int] NOT NULL,
	[type] [int] NOT NULL,
	[createtime] [bigint] NOT NULL,
	[updatetime] [bigint] NOT NULL,
 CONSTRAINT [PK_channels_info] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[device_param]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[device_param](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[sn] [nvarchar](16) NOT NULL,
	[porttype] [int] NOT NULL,
	[heart_beat] [int] NULL,
	[analog_upload_cycle] [int] NOT NULL,
	[low] [float] NOT NULL,
	[high] [float] NOT NULL,
	[unit] [nvarchar](10) NOT NULL,
	[fail_times] [int] NOT NULL,
	[alarm_1] [int] NOT NULL,
	[alarm_2] [int] NOT NULL,
	[alarm_3] [int] NOT NULL,
	[updatetime] [bigint] NOT NULL,
	[createtime] [bigint] NOT NULL,
 CONSTRAINT [PK__DEVICE_P__3213E83F042D4E77] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[devices_info]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
SET ANSI_PADDING ON
GO
CREATE TABLE [dbo].[devices_info](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[parent_id] [int] NOT NULL,
	[sn] [nvarchar](20) NOT NULL,
	[mac] [varchar](20) NULL,
	[ip] [char](15) NOT NULL,
	[type] [nchar](10) NOT NULL,
	[status] [char](1) NOT NULL,
	[level] [int] NOT NULL,
	[createtime] [bigint] NOT NULL,
	[updatetime] [bigint] NOT NULL,
 CONSTRAINT [PK_devices_info] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
SET ANSI_PADDING OFF
GO
/****** Object:  Table [dbo].[mpoint]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[mpoint](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[pid] [int] NOT NULL,
	[ciid] [int] NOT NULL,
	[name] [nchar](20) NOT NULL,
	[starttime] [bigint] NOT NULL,
	[endtime] [bigint] NOT NULL,
 CONSTRAINT [PK_mpoint] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[mpoint_data]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[mpoint_data](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[mpoint_id] [int] NOT NULL,
	[swiftnum] [int] NOT NULL,
	[data] [float] NOT NULL,
	[time] [bigint] NOT NULL,
	[createtime] [bigint] NOT NULL,
 CONSTRAINT [PK_mpoint_data] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[mpoint_pre_status]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[mpoint_pre_status](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[mpoint_id] [int] NOT NULL,
	[swiftnum] [int] NOT NULL,
	[raw_status] [int] NOT NULL,
	[real_status] [int] NOT NULL,
	[alarm_level] [int] NOT NULL,
	[time] [bigint] NOT NULL,
	[createtime] [bigint] NOT NULL,
 CONSTRAINT [PK_mpoint_pre_status] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[mpoint_realtime_status]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[mpoint_realtime_status](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[mpoint_id] [int] NOT NULL,
	[swiftnum] [int] NOT NULL,
	[raw_status] [int] NOT NULL,
	[real_status] [int] NOT NULL,
	[alarm_level] [int] NOT NULL,
	[time] [bigint] NOT NULL,
	[createtime] [bigint] NOT NULL,
 CONSTRAINT [PK_mpoint_realtime_status] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[mpoint_status]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[mpoint_status](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[mpoint_id] [int] NOT NULL,
	[swiftnum] [int] NOT NULL,
	[raw_status] [int] NOT NULL,
	[real_status] [int] NOT NULL,
	[alarm_level] [int] NOT NULL,
	[time] [bigint] NOT NULL,
	[createtime] [bigint] NOT NULL,
 CONSTRAINT [PK_mpoint_status] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[place]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[place](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[pid] [int] NOT NULL,
	[name] [nchar](20) NOT NULL,
	[level] [int] NOT NULL,
	[del] [int] NULL,
 CONSTRAINT [PK_place] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[push_list]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
SET ANSI_PADDING ON
GO
CREATE TABLE [dbo].[push_list](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[user_id] [int] NOT NULL,
	[push_content] [varchar](50) NOT NULL,
	[createtime] [bigint] NOT NULL,
 CONSTRAINT [PK_push_list] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
SET ANSI_PADDING OFF
GO
/****** Object:  Table [dbo].[user_default_report_setting]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[user_default_report_setting](
	[id] [int] NOT NULL,
	[user_id] [int] NOT NULL,
	[pid] [int] NOT NULL,
 CONSTRAINT [PK_user_default_report_setting] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[webservice_user]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[webservice_user](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[username] [nvarchar](50) NOT NULL,
	[password] [nvarchar](100) NOT NULL,
	[email] [nvarchar](50) NOT NULL,
	[apikey] [nvarchar](100) NOT NULL,
	[roles] [nvarchar](50) NOT NULL,
	[expireTime] [bigint] NOT NULL,
 CONSTRAINT [PK_webservice_user] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[ws_worktime_daily]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
SET ANSI_PADDING ON
GO
CREATE TABLE [dbo].[ws_worktime_daily](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[device_id] [int] NOT NULL,
	[worktime] [varchar](500) NOT NULL,
	[createtime] [bigint] NOT NULL,
 CONSTRAINT [PK_ws_worktime_daily] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
SET ANSI_PADDING OFF
GO
/****** Object:  View [dbo].[chan_status_without_online]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO




CREATE VIEW [dbo].[chan_status_without_online] AS
SELECT
	plant,
	workshop,
	region,
	line,
	station,
	mpoint,
	sn, slot, port, porttype,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, ciid, pid, time, status
FROM (
	SELECT
		p0.name AS plant,
		p1.name AS workshop,
		p2.name AS region,
		p3.name AS line,
		p4.name AS station,
		mp.name AS mpoint,
		di.sn, ci.slot, ci.port, ci.type as porttype,
		p0.id AS plantid,
		p1.id AS workshopid,
		p2.id AS regionid,
		p3.id AS lineid,
		p4.id AS stationid,
		ms.id, mp.ciid, mp.pid, time,
		LAG(raw_status) OVER (PARTITION BY ms.mpoint_id ORDER BY time) AS pstatus,
		raw_status as status,
		LEAD(raw_status) OVER (PARTITION BY ms.mpoint_id ORDER BY time) AS nstatus
	FROM mpoint_status AS ms
	LEFT JOIN mpoint AS mp
	ON ms.mpoint_id = mp.id
	LEFT JOIN channels_info AS ci
	ON mp.ciid = ci.id
	LEFT JOIN devices_info AS di
	ON ci.device_id = di.id
	LEFT JOIN place AS p4
	ON mp.pid = p4.id
	LEFT JOIN place AS p3
	ON p4.pid = p3.id
	LEFT JOIN place AS p2
	ON p3.pid = p2.id
	LEFT JOIN place AS p1
	ON p2.pid = p1.id
	LEFT JOIN place AS p0
	ON p1.pid = p0.id
) AS t
WHERE status != 259 AND (status != 256 OR (nstatus IS NULL OR nstatus = 260)) AND status != 260






GO
/****** Object:  View [dbo].[chan_status_without_online_level]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO



CREATE VIEW [dbo].[chan_status_without_online_level] AS
SElECT
	plant,
	workshop,
	region,
	line,
	station,
	mpoint,
	sn, slot, port, porttype,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, ciid, pid, time,
CASE
	WHEN status > 255 THEN status
	ELSE
	CASE
		WHEN porttype = 9 THEN
		CASE
			WHEN status & 0x80 = 0x80 THEN 0x80
			WHEN status & 0x50 = 0x50 THEN 0x50
			WHEN status & 0x40 = 0x40 THEN 0x40
			WHEN status & 0x20 = 0x20 THEN 0x20
			ELSE 0x00
		END
		ELSE
		CASE
			WHEN status & 0x20 = 0x20 THEN 0x20
			ELSE 0x00
		END
	END
END AS status
FROM chan_status_without_online





GO
/****** Object:  View [dbo].[chan_status_without_online_level_no_dup]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO




CREATE VIEW [dbo].[chan_status_without_online_level_no_dup] AS
SELECT
plant,
	workshop,
	region,
	line,
	station,
	mpoint,
	sn, slot, port, porttype,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, ciid, pid, time, status
FROM (
	SELECT
	plant,
	workshop,
	region,
	line,
	station,
	mpoint,
	sn, slot, port, porttype,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, ciid, pid, time,
	LAG(status) OVER (PARTITION BY ciid, pid ORDER BY time) AS pstatus,
	status,
	LEAD(status) OVER (PARTITION BY ciid, pid ORDER BY time) AS nstatus
	FROM chan_status_without_online_level
) AS t
WHERE pstatus IS NULL OR pstatus != status






GO
/****** Object:  View [dbo].[chan_status]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO



CREATE VIEW [dbo].[chan_status] AS
SELECT
	plant,
	workshop,
	region,
	line,
	station,
	mpoint,
	sn, slot, port, porttype,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, ciid, pid,
	time AS starttime,
	LEAD(time) OVER (PARTITION BY ciid, pid ORDER BY time) AS endtime,
	status
FROM chan_status_without_online_level_no_dup





GO
/****** Object:  View [dbo].[view_mpoint_status]    Script Date: 2018/7/27 16:06:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO

CREATE view [dbo].[view_mpoint_status]
as
select id, mpoint_id, real_status, alarm_level,time as starttime,
	lead(time,1,0) over(partition by mpoint_id order by time) as endtime
from mpoint_status


GO
USE [master]
GO
ALTER DATABASE [hzwiot] SET  READ_WRITE 
GO
