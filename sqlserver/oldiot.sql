USE [master]
GO
/****** Object:  Database [hzwiot]    Script Date: 2018/7/27 15:26:08 ******/
CREATE DATABASE [hzwiot]
 CONTAINMENT = NONE
 ON  PRIMARY 
( NAME = N'hzwIOT', FILENAME = N'C:\Program Files\Microsoft SQL Server\MSSQL11.MSSQLSERVER\MSSQL\DATA\hzwIOT.mdf' , SIZE = 231488KB , MAXSIZE = UNLIMITED, FILEGROWTH = 1024KB )
 LOG ON 
( NAME = N'hzwIOT_log', FILENAME = N'C:\Program Files\Microsoft SQL Server\MSSQL11.MSSQLSERVER\MSSQL\DATA\hzwIOT_log.ldf' , SIZE = 16278784KB , MAXSIZE = 2048GB , FILEGROWTH = 10%)
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
/****** Object:  StoredProcedure [dbo].[clearDb]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
-- =============================================
-- Author:		<Author,,Name>
-- Create date: <Create Date,,>
-- Description:	<Description,,>
-- =============================================
CREATE PROCEDURE [dbo].[clearDb]
AS
BEGIN
	SET NOCOUNT ON;

	DELETE FROM channel_ad
	DELETE FROM channel_info
	DELETE FROM channel_status
	DELETE FROM date_status
	DELETE FROM device_info
	DELETE FROM device_param
	DELETE FROM mpoint
	DELETE FROM place
	DELETE FROM pre_status
	DELETE FROM realtime_status
	DELETE FROM STATION_STATISTICS_PER_DAY
END

GO
/****** Object:  StoredProcedure [dbo].[delete_mpoint]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
-- =============================================
-- Author:		wujun
-- Create date: 2018-06-13
-- Description:	删除监控点-》删除channel_status->channel_ad->realtime_status->pre_status
-- =============================================
CREATE PROCEDURE [dbo].[delete_mpoint]
 @sn NVARCHAR(100)

AS
BEGIN

	DELETE cs
	FROM channel_status AS cs
	LEFT JOIN channel_info AS ci
	ON cs.ciid = ci.id
	WHERE ci.sn = @sn
	 
	DELETE ca
	FROM channel_ad AS ca
	LEFT JOIN channel_info AS ci
	ON ca.ciid = ci.id
	WHERE ci.sn = @sn

	DELETE ra
	FROM realtime_status AS ra
	LEFT JOIN channel_info AS ci
	ON ra.ciid = ci.id
	WHERE ci.sn = @sn

	DELETE pa
	FROM pre_status AS pa
	LEFT JOIN channel_info AS ci
	ON pa.ciid = ci.id
	WHERE ci.sn = @sn

	DELETE mp
	FROM mpoint AS mp
	LEFT JOIN channel_info AS ci
	ON mp.ciid = ci.id
	WHERE ci.sn = @sn
END
GO
/****** Object:  StoredProcedure [dbo].[delete_mpoint_one]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
-- =============================================
-- Author:		wujun
-- Create date: 2018-06-13
-- Description:	删除监控点-》删除channel_status->channel_ad->realtime_status->pre_status
-- =============================================
CREATE PROCEDURE [dbo].[delete_mpoint_one]
 @id int

AS
BEGIN

	DELETE cs
	FROM channel_status AS cs
	LEFT JOIN mpoint AS ci
	ON cs.ciid = ci.ciid
	WHERE ci.id = @id
	 
	DELETE ca
	FROM channel_ad AS ca
	LEFT JOIN mpoint AS ci
	ON ca.ciid = ci.ciid
	WHERE ci.id = @id

	DELETE ra
	FROM realtime_status AS ra
	LEFT JOIN mpoint AS ci
	ON ra.ciid = ci.ciid
	WHERE ci.id = @id

	DELETE pa
	FROM pre_status AS pa
	LEFT JOIN mpoint AS ci
	ON pa.ciid = ci.ciid
	WHERE ci.id = @id

	DELETE mp
	FROM mpoint AS mp
	LEFT JOIN mpoint AS ci
	ON mp.ciid = ci.ciid
	WHERE ci.id = @id
END
GO
/****** Object:  StoredProcedure [dbo].[new_channel_data]    Script Date: 2018/7/27 15:26:08 ******/
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

	EXEC updateOfflineTime @sn, @slot, @port, @type, @time

	INSERT INTO channel_ad
	(ciid, pid, ad_value, time)
	VALUES
	(@ciid, @pid, @data, @time)
END

GO
/****** Object:  StoredProcedure [dbo].[new_channel_status]    Script Date: 2018/7/27 15:26:08 ******/
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

	IF @nstatus = @ONLINE
	BEGIN
		UPDATE realtime_status
		SET status = @pstatus,
		    time = @ptime
		WHERE status = @OFFLINE
		  AND id = @rtsid
	END
	ELSE
	BEGIN
		IF @ntime > @ctime AND @nstatus != @POWERON
		BEGIN
			UPDATE realtime_status
			SET status = @nstatus,
			    time = @ntime
			WHERE id = @rtsid
			
			IF @nstatus != @POWEROFF
			BEGIN
				UPDATE pre_status
				SET status = @nstatus,
				    time = @ntime
				WHERE id = @psid
			END
		END
	END

	IF @nstatus < @OFFLINE
	BEGIN
		EXEC updateOfflineTime @sn, @slot, @port, @type, @ntime
	END

	INSERT INTO channel_status
	(ciid, pid, status, time)
	VALUES
	(@ciid, @pid, @nstatus, @ntime)
END

GO
/****** Object:  StoredProcedure [dbo].[new_device_status]    Script Date: 2018/7/27 15:26:08 ******/
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
/****** Object:  StoredProcedure [dbo].[new_mpoint]    Script Date: 2018/7/27 15:26:08 ******/
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
/****** Object:  StoredProcedure [dbo].[pageTest]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[pageTest] --用于翻页的测试 
--需要把排序字段放在第一列 

( 
@FirstID nvarchar(20)=null, --当前页面里的第一条记录的排序字段的值 
@LastID nvarchar(20)=null, --当前页面里的最后一条记录的排序字段的值 
@isNext bit=null, --true 1 :下一页；false 0:上一页 
@allCount int output, --返回总记录数 
@pageSize int output, --返回一页的记录数 
@CurPage int --页号（第几页）0：第一页；-1最后一页。 
) 

AS 

if @CurPage=0--表示第一页 
begin 
--统计总记录数 
select @allCount=count(id) from channel_ad 

set @pageSize=10 
--返回第一页的数据 
select top 10 
id, 
ciid, 
ad_value 
from channel_ad order by id 
end 

else if @CurPage=-1--表示最后一页 

select * from 
(select top 10 id, 
ciid, 
ad_value 

from channel_ad order by id desc ) as aa 
order by id 
else 

begin 
if @isNext=1 
--翻到下一页 
select top 10 id, 
ciid, 
ad_value 
from channel_ad where id > @LastID order by id 
else 
--翻到上一页 
select * from 
(select top 10 id, 
ciid, 
ad_value
from channel_ad where id < @FirstID order by id desc) as bb order by id 
end
GO
/****** Object:  StoredProcedure [dbo].[scpage]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[scpage] (
    @recordTotal INT OUTPUT,            --输出记录总数
    @viewName VARCHAR(800),        --表名
    @fieldName VARCHAR(800) = '*',        --查询字段
    @keyName VARCHAR(200) = 'id',            --索引字段
    @pageSize INT = 20,                    --每页记录数
    @pageNo INT =1,                    --当前页
    @orderString VARCHAR(200),        --排序条件
    @whereString VARCHAR(800) = '1=1'        --WHERE条件
)

AS

BEGIN
     DECLARE @beginRow INT
     DECLARE @endRow INT
     DECLARE @tempLimit VARCHAR(200)
     DECLARE @tempCount NVARCHAR(1000)
     DECLARE @tempMain VARCHAR(1000)
     --declare @timediff datetime 
     
     set nocount on
     --select @timediff=getdate() --记录时间

     SET @beginRow = (@pageNo - 1) * @pageSize    + 1
     SET @endRow = @pageNo * @pageSize
     SET @tempLimit = 'rows BETWEEN ' + CAST(@beginRow AS VARCHAR) +' AND '+CAST(@endRow AS VARCHAR)
     
     --输出参数为总记录数
     SET @tempCount = 'SELECT @recordTotal = COUNT(*) FROM (SELECT '+@keyName+' FROM '+@viewName+' WHERE '+@whereString+') AS my_temp'
     EXECUTE sp_executesql @tempCount,N'@recordTotal INT OUTPUT',@recordTotal OUTPUT
       
     --主查询返回结果集
     SET @tempMain = 'SELECT * FROM (SELECT ROW_NUMBER() OVER (order by '+@orderString+') AS rows ,'+@fieldName+' FROM '+@viewName+' WHERE '+@whereString+') AS main_temp WHERE '+@tempLimit
     
     --PRINT @tempMain
     EXECUTE (@tempMain)
     --select datediff(ms,@timediff,getdate()) as 耗时 
     
     set nocount off
END
GO
/****** Object:  StoredProcedure [dbo].[updateOfflineTime]    Script Date: 2018/7/27 15:26:08 ******/
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

	SELECT TOP 1 @offlineTime = cs.time
	FROM channel_status AS cs
	LEFT JOIN channel_info AS ci
	ON cs.ciid = ci.id
	WHERE ci.sn = @sn
	  AND ci.slot = @slot
	  AND ci.port = @port
	  AND ci.porttype = @type
	  AND cs.status = @OFFLINE
	  AND cs.time < @time
	ORDER BY time DESC

	SELECT TOP 1 @poweronTime = cs.time
	FROM channel_status AS cs
	LEFT JOIN channel_info AS ci
	ON cs.ciid = ci.id
	WHERE ci.sn = @sn
	  AND ci.slot = @slot
	  AND ci.port = @port
	  AND ci.porttype = @type
	  AND cs.status = @POWERON
	  AND cs.time > @time
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

	UPDATE cs
	SET cs.time = @time
	FROM channel_status AS cs
	LEFT JOIN channel_info AS ci
	ON cs.ciid = ci.id
	WHERE ci.sn = @sn
	  AND cs.status = @OFFLINE
	  AND cs.time = @offlineTime
END

GO
/****** Object:  Table [dbo].[channel_ad]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[channel_ad](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[ciid] [int] NOT NULL,
	[pid] [int] NOT NULL,
	[ad_value] [float] NOT NULL,
	[time] [bigint] NOT NULL,
 CONSTRAINT [PK_channel_ad1] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[channel_info]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[channel_info](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[sn] [nvarchar](20) NOT NULL,
	[slot] [int] NOT NULL,
	[port] [int] NOT NULL,
	[porttype] [int] NOT NULL,
	[createtime] [bigint] NOT NULL,
	[updatetime] [bigint] NOT NULL,
PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[channel_status]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[channel_status](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[ciid] [int] NOT NULL,
	[pid] [int] NOT NULL,
	[status] [int] NOT NULL,
	[time] [bigint] NOT NULL,
 CONSTRAINT [PK_channel_status] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[date_status]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[date_status](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[pid] [int] NOT NULL,
	[ciid] [int] NOT NULL,
	[date] [bigint] NOT NULL,
	[status] [int] NOT NULL,
	[times] [int] NOT NULL,
	[durationtime] [int] NOT NULL,
	[yield] [float] NOT NULL,
 CONSTRAINT [PK_date_status] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[device_info]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[device_info](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[sn] [nvarchar](20) NOT NULL,
	[ip] [nvarchar](20) NOT NULL,
	[psn] [nvarchar](20) NULL,
	[devicetype] [nvarchar](20) NOT NULL,
	[createtime] [bigint] NOT NULL,
	[updatetime] [bigint] NOT NULL,
 CONSTRAINT [PK_device_info] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[device_param]    Script Date: 2018/7/27 15:26:08 ******/
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
/****** Object:  Table [dbo].[fos_group]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
SET ANSI_PADDING ON
GO
CREATE TABLE [dbo].[fos_group](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[name] [nvarchar](180) NOT NULL,
	[roles] [varchar](max) NOT NULL,
PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY] TEXTIMAGE_ON [PRIMARY]

GO
SET ANSI_PADDING OFF
GO
/****** Object:  Table [dbo].[FOS_USER]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
SET ANSI_PADDING ON
GO
CREATE TABLE [dbo].[FOS_USER](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[username] [nvarchar](180) NOT NULL,
	[username_canonical] [nvarchar](180) NOT NULL,
	[email] [nvarchar](180) NOT NULL,
	[email_canonical] [nvarchar](180) NOT NULL,
	[enabled] [bit] NOT NULL,
	[salt] [nvarchar](255) NULL,
	[password] [nvarchar](255) NOT NULL,
	[last_login] [datetime2](6) NULL,
	[confirmation_token] [nvarchar](180) NULL,
	[password_requested_at] [datetime2](6) NULL,
	[roles] [varchar](max) NOT NULL,
PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY] TEXTIMAGE_ON [PRIMARY]

GO
SET ANSI_PADDING OFF
GO
/****** Object:  Table [dbo].[fos_user_group]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[fos_user_group](
	[user_id] [int] NOT NULL,
	[group_id] [int] NOT NULL,
PRIMARY KEY CLUSTERED 
(
	[user_id] ASC,
	[group_id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[mpoint]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[mpoint](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[pid] [int] NOT NULL,
	[ciid] [int] NOT NULL,
	[name] [nvarchar](20) NOT NULL,
	[createtime] [bigint] NOT NULL,
	[updatetime] [bigint] NOT NULL,
PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[place]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[place](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[pid] [int] NOT NULL,
	[name] [nvarchar](20) NOT NULL,
	[level] [int] NOT NULL,
PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[pre_status]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[pre_status](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[ciid] [int] NOT NULL,
	[pid] [int] NOT NULL,
	[status] [int] NOT NULL,
	[time] [bigint] NOT NULL,
 CONSTRAINT [PK_pre_status] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[realtime_status]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[realtime_status](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[ciid] [int] NOT NULL,
	[pid] [int] NOT NULL,
	[status] [int] NOT NULL,
	[time] [bigint] NOT NULL,
 CONSTRAINT [PK_realtime_status] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY],
 CONSTRAINT [IX_realtime_status] UNIQUE NONCLUSTERED 
(
	[ciid] ASC,
	[pid] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[STATION_STATISTICS_PER_DAY]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[STATION_STATISTICS_PER_DAY](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[pid] [int] NOT NULL,
	[date] [bigint] NOT NULL,
	[qualified_rate] [float] NOT NULL,
	[failure_rate] [float] NOT NULL,
	[porttype] [int] NOT NULL,
PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[user_manage]    Script Date: 2018/7/27 15:26:08 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[user_manage](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[username] [nvarchar](50) NOT NULL,
	[password] [nvarchar](50) NOT NULL,
 CONSTRAINT [PK_user] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[webservice_user]    Script Date: 2018/7/27 15:26:08 ******/
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
	[expireTime] [int] NOT NULL,
 CONSTRAINT [PK_webservice_user] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  View [dbo].[chan_status_without_online]    Script Date: 2018/7/27 15:26:08 ******/
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
		ci.sn, ci.slot, ci.port, ci.porttype,
		p0.id AS plantid,
		p1.id AS workshopid,
		p2.id AS regionid,
		p3.id AS lineid,
		p4.id AS stationid,
		cs.id, cs.ciid, cs.pid, time,
		LAG(status) OVER (PARTITION BY cs.ciid, cs.pid ORDER BY time) AS pstatus,
		status,
		LEAD(status) OVER (PARTITION BY cs.ciid, cs.pid ORDER BY time) AS nstatus
	FROM channel_status AS cs
	LEFT JOIN mpoint AS mp
	ON cs.ciid = mp.ciid AND cs.pid = mp.pid
	LEFT JOIN channel_info AS ci
	ON mp.ciid = ci.id
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
/****** Object:  View [dbo].[chan_status_without_online_level]    Script Date: 2018/7/27 15:26:08 ******/
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
/****** Object:  View [dbo].[chan_status_without_online_level_no_dup]    Script Date: 2018/7/27 15:26:08 ******/
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
/****** Object:  View [dbo].[chan_status]    Script Date: 2018/7/27 15:26:08 ******/
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
/****** Object:  Index [UNIQ_84D77E98B13102F1]    Script Date: 2018/7/27 15:26:08 ******/
CREATE UNIQUE NONCLUSTERED INDEX [UNIQ_84D77E98B13102F1] ON [dbo].[mpoint]
(
	[ciid] ASC
)
WHERE ([ciid] IS NOT NULL)
WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, SORT_IN_TEMPDB = OFF, IGNORE_DUP_KEY = OFF, DROP_EXISTING = OFF, ONLINE = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
GO
ALTER TABLE [dbo].[fos_user_group]  WITH CHECK ADD  CONSTRAINT [FK_583D1F3EA76ED395] FOREIGN KEY([user_id])
REFERENCES [dbo].[FOS_USER] ([id])
GO
ALTER TABLE [dbo].[fos_user_group] CHECK CONSTRAINT [FK_583D1F3EA76ED395]
GO
ALTER TABLE [dbo].[fos_user_group]  WITH CHECK ADD  CONSTRAINT [FK_583D1F3EFE54D947] FOREIGN KEY([group_id])
REFERENCES [dbo].[fos_group] ([id])
GO
ALTER TABLE [dbo].[fos_user_group] CHECK CONSTRAINT [FK_583D1F3EFE54D947]
GO
USE [master]
GO
ALTER DATABASE [hzwiot] SET  READ_WRITE 
GO
