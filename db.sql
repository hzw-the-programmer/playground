USE [master]
GO
/****** Object:  Database [v2_1_1]    Script Date: 2017/12/4 14:52:41 ******/
CREATE DATABASE [v2_1_1]
 CONTAINMENT = NONE
 ON  PRIMARY
( NAME = N'v2_1_1', FILENAME = N'D:\v2_1_1.mdf' , SIZE = 5120KB , MAXSIZE = UNLIMITED, FILEGROWTH = 1024KB )
 LOG ON
( NAME = N'v2_1_1_log', FILENAME = N'D:\v2_1_1_log.ldf' , SIZE = 5696KB , MAXSIZE = 2048GB , FILEGROWTH = 10%)
GO
ALTER DATABASE [v2_1_1] SET COMPATIBILITY_LEVEL = 110
GO
IF (1 = FULLTEXTSERVICEPROPERTY('IsFullTextInstalled'))
begin
EXEC [v2_1_1].[dbo].[sp_fulltext_database] @action = 'enable'
end
GO
ALTER DATABASE [v2_1_1] SET ANSI_NULL_DEFAULT OFF
GO
ALTER DATABASE [v2_1_1] SET ANSI_NULLS OFF
GO
ALTER DATABASE [v2_1_1] SET ANSI_PADDING OFF
GO
ALTER DATABASE [v2_1_1] SET ANSI_WARNINGS OFF
GO
ALTER DATABASE [v2_1_1] SET ARITHABORT OFF
GO
ALTER DATABASE [v2_1_1] SET AUTO_CLOSE OFF
GO
ALTER DATABASE [v2_1_1] SET AUTO_CREATE_STATISTICS ON
GO
ALTER DATABASE [v2_1_1] SET AUTO_SHRINK OFF
GO
ALTER DATABASE [v2_1_1] SET AUTO_UPDATE_STATISTICS ON
GO
ALTER DATABASE [v2_1_1] SET CURSOR_CLOSE_ON_COMMIT OFF
GO
ALTER DATABASE [v2_1_1] SET CURSOR_DEFAULT  GLOBAL
GO
ALTER DATABASE [v2_1_1] SET CONCAT_NULL_YIELDS_NULL OFF
GO
ALTER DATABASE [v2_1_1] SET NUMERIC_ROUNDABORT OFF
GO
ALTER DATABASE [v2_1_1] SET QUOTED_IDENTIFIER OFF
GO
ALTER DATABASE [v2_1_1] SET RECURSIVE_TRIGGERS OFF
GO
ALTER DATABASE [v2_1_1] SET  DISABLE_BROKER
GO
ALTER DATABASE [v2_1_1] SET AUTO_UPDATE_STATISTICS_ASYNC OFF
GO
ALTER DATABASE [v2_1_1] SET DATE_CORRELATION_OPTIMIZATION OFF
GO
ALTER DATABASE [v2_1_1] SET TRUSTWORTHY OFF
GO
ALTER DATABASE [v2_1_1] SET ALLOW_SNAPSHOT_ISOLATION OFF
GO
ALTER DATABASE [v2_1_1] SET PARAMETERIZATION SIMPLE
GO
ALTER DATABASE [v2_1_1] SET READ_COMMITTED_SNAPSHOT OFF
GO
ALTER DATABASE [v2_1_1] SET HONOR_BROKER_PRIORITY OFF
GO
ALTER DATABASE [v2_1_1] SET RECOVERY FULL
GO
ALTER DATABASE [v2_1_1] SET  MULTI_USER
GO
ALTER DATABASE [v2_1_1] SET PAGE_VERIFY CHECKSUM
GO
ALTER DATABASE [v2_1_1] SET DB_CHAINING OFF
GO
ALTER DATABASE [v2_1_1] SET FILESTREAM( NON_TRANSACTED_ACCESS = OFF )
GO
ALTER DATABASE [v2_1_1] SET TARGET_RECOVERY_TIME = 0 SECONDS
GO
EXEC sys.sp_db_vardecimal_storage_format N'v2_1_1', N'ON'
GO
USE [v2_1_1]
GO
/****** Object:  StoredProcedure [dbo].[clear]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[clear]

AS
BEGIN
	DELETE FROM channel_changes
	DELETE FROM channel_data
	DELETE FROM channel_status
	DELETE FROM channels
	DELETE FROM devices
	DELETE FROM equipments
	DELETE FROM place_coordinates
	DELETE FROM places
	DELETE FROM places_users
	DELETE FROM realtime_status
END

GO
/****** Object:  StoredProcedure [dbo].[create_channel]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[create_channel]
	@sn bigint,
	@slot smallint,
	@port smallint,
	@type smallint,
	@pname nvarchar(50),
	@parent_pid bigint,
	@left smallint,
	@top smallint,
	@right smallint,
	@bottom smallint,
	@datetime datetime
AS
	DECLARE @did bigint

	SELECT @did = id FROM devices WHERE sn = @sn

	IF @did IS NULL
		RETURN 1

	DECLARE @cid bigint

	SELECT @cid = id FROM channels
	WHERE did = @did AND
	      slot = @slot AND
		  port = @port AND
		  type = @type

	IF @cid IS NULL
	BEGIN
		INSERT INTO channels
		(did, slot, port, type)
		VALUES
		(@did, @slot, @port, @type)

		SELECT @cid = SCOPE_IDENTITY()
	END

	DECLARE @pid bigint

	SELECT @pid = id FROM places
	WHERE name = @pname AND
		  pid = @parent_pid

	IF @pid IS NULL
	BEGIN
		DECLARE @level tinyint

		SELECT @level = level FROM places WHERE id = @parent_pid

		IF @level IS NULL
			RETURN 1

		INSERT INTO places
		(name, pid, level)
		VALUES
		(@pname, @parent_pid, @level + 1)

		SELECT @pid = SCOPE_IDENTITY()
	END

	--UPDATE channel_changes
	--SET endtime = @datetime
	--WHERE endtime IS NULL AND
	--	  cid = @cid AND
	--	  pid = @pid

	INSERT INTO channel_changes
	(cid, pid, starttime)
	VALUES
	(@cid, @pid, @datetime)

	INSERT INTO place_coordinates
	(pid, [left], [top], [right], bottom)
	VALUES
	(@pid, @left, @top, @right, @bottom)

	DECLARE @sid smallint

	SELECT @sid = id FROM status WHERE name = 'ON'

	INSERT INTO realtime_status
	(cid, pid, ctime, status)
	VALUES
	(@cid, @pid, @datetime, @sid)

	INSERT INTO channel_status
	(cid, pid, ctime, status)
	VALUES
	(@cid, @pid, @datetime, @sid)





GO
/****** Object:  StoredProcedure [dbo].[create_place]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[create_place]
	@pname nvarchar(50),
	@parent_pid bigint,
	@left smallint,
	@top smallint,
	@right smallint,
	@bottom smallint,
	@workid varchar(50),
	@datetime datetime
AS
	DECLARE @pid bigint

	SELECT @pid = id FROM places
	WHERE name = @pname AND
	      pid = @parent_pid

	IF @pid IS NULL
	BEGIN
		DECLARE @level tinyint

		SELECT @level = level FROM places WHERE id = @parent_pid

		IF @level IS NULL
			RETURN 1

		INSERT INTO places
		(name, pid, level)
		VALUES
		(@pname, @parent_pid, @level + 1)

		SELECT @pid = SCOPE_IDENTITY()
	END

	INSERT INTO place_coordinates
	(pid, [left], [top], [right], bottom)
	VALUES
	(@pid, @left, @top, @right, @bottom)

	DECLARE @uid bigint

	SELECT @uid = id FROM users WHERE workid = @workid

	IF @uid IS NOT NULL
	BEGIN
		UPDATE places_users
		SET endtime = @datetime
		WHERE endtime IS NULL AND
			  pid = @pid AND
			  uid = @uid


		INSERT INTO places_users
		(pid, uid, starttime)
		VALUES
		(@pid, @uid, @datetime)
	END




GO
/****** Object:  StoredProcedure [dbo].[delete_channel]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[delete_channel]
	@cid bigint,
	@datetime datetime
AS
	DECLARE @pid bigint

	SELECT @pid = pid FROM channel_changes
	WHERE endtime IS NULL AND
		  cid = @cid

	IF @pid IS NULL
		RETURN 1

	UPDATE channel_changes
	SET endtime = @datetime
	WHERE endtime IS NULL AND
	      cid = @cid AND
		  pid = @pid

	DELETE FROM place_coordinates WHERE pid = @pid

	DECLARE @sid smallint

	SELECT @sid = id FROM status WHERE name = 'OFF'

	DELETE FROM realtime_status
	WHERE cid = @cid AND
	      pid = @pid

	INSERT INTO channel_status
	(cid, pid, ctime, status)
	VALUES
	(@cid, @pid, @datetime, @sid)




GO
/****** Object:  StoredProcedure [dbo].[delete_place]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[delete_place]
	@pid bigint,
	@datetime datetime
AS
	DELETE pc
	FROM place_coordinates AS pc
	LEFT JOIN
	places AS p
	ON
	pc.pid = p.id
	LEFT JOIN
	places AS p1
	ON
	p.pid = p1.id
	LEFT JOIN
	places AS p2
	ON
	p1.pid = p2.id
	LEFT JOIN
	places AS p3
	ON
	p2.pid = p3.id
	LEFT JOIN
	places AS p4
	ON
	p3.pid = p4.id
	LEFT JOIN
	places AS p5
	ON
	p4.pid = p5.id
	LEFT JOIN
	places AS p6
	ON
	p5.pid = p6.id
	WHERE p6.id IS NULL AND (
	p.id = @pid OR
	p1.id = @pid OR
	p2.id = @pid OR
	p3.id = @pid OR
	p4.id = @pid OR
	p5.id = @pid OR
	p6.id = @pid
	)

	DELETE e
	FROM equipments AS e
	LEFT JOIN
	places AS p
	ON
	e.pid = p.id
	LEFT JOIN
	places AS p1
	ON
	p.pid = p1.id
	LEFT JOIN
	places AS p2
	ON
	p1.pid = p2.id
	LEFT JOIN
	places AS p3
	ON
	p2.pid = p3.id
	LEFT JOIN
	places AS p4
	ON
	p3.pid = p4.id
	LEFT JOIN
	places AS p5
	ON
	p4.pid = p5.id
	LEFT JOIN
	places AS p6
	ON
	p5.pid = p6.id
	WHERE
	p.id = @pid OR
	p1.id = @pid OR
	p2.id = @pid OR
	p3.id = @pid OR
	p4.id = @pid OR
	p5.id = @pid OR
	p6.id = @pid

	--UPDATE cc
	--SET cc.endtime = @datetime
	--FROM channel_changes AS cc
	--LEFT JOIN
	--places AS p
	--ON
	--cc.pid = p.id
	--LEFT JOIN
	--places AS p1
	--ON
	--p.pid = p1.id
	--LEFT JOIN
	--places AS p2
	--ON
	--p1.pid = p2.id
	--LEFT JOIN
	--places AS p3
	--ON
	--p2.pid = p3.id
	--LEFT JOIN
	--places AS p4
	--ON
	--p3.pid = p4.id
	--LEFT JOIN
	--places AS p5
	--ON
	--p4.pid = p5.id
	--LEFT JOIN
	--places AS p6
	--ON
	--p5.pid = p6.id
	--WHERE cc.endtime IS NULL AND (
	--p.id = @pid OR
	--p1.id = @pid OR
	--p2.id = @pid OR
	--p3.id = @pid OR
	--p4.id = @pid OR
	--p5.id = @pid OR
	--p6.id = @pid
	--)

	UPDATE pu
	SET pu.endtime = @datetime
	FROM places_users AS pu
	LEFT JOIN
	places AS p
	ON
	pu.pid = p.id
	LEFT JOIN
	places AS p1
	ON
	p.pid = p1.id
	LEFT JOIN
	places AS p2
	ON
	p1.pid = p2.id
	LEFT JOIN
	places AS p3
	ON
	p2.pid = p3.id
	LEFT JOIN
	places AS p4
	ON
	p3.pid = p4.id
	LEFT JOIN
	places AS p5
	ON
	p4.pid = p5.id
	LEFT JOIN
	places AS p6
	ON
	p5.pid = p6.id
	WHERE endtime IS NULL AND (
	p.id = @pid OR
	p1.id = @pid OR
	p2.id = @pid OR
	p3.id = @pid OR
	p4.id = @pid OR
	p5.id = @pid OR
	p6.id = @pid
	)



GO
/****** Object:  StoredProcedure [dbo].[get_on_channels]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[get_on_channels]
AS
BEGIN
	SELECT
	d.id AS did, d.sn, d.ip, d.port,
	c.id AS cid, c.slot, c.port AS cport, c.type,
	p.id AS pid, p.name, p.pid AS parentid, p.level,
	pc.[left], pc.[top], pc.[right], pc.bottom,
	root.id AS rootid,
	plant.id AS plantid,
	workshop.id AS workshopid,
	region.id AS regionid,
	line.id AS lineid,
	station.id AS stationid
	FROM
	realtime_status AS rs
	LEFT JOIN
	channels AS c
	ON rs.cid = c.id
	LEFT JOIN
	devices AS d
	ON c.did = d.id
	LEFT JOIN
	places AS p
	ON rs.pid = p.id
	LEFT JOIN
	place_coordinates AS pc
	ON p.id = pc.pid
	LEFT JOIN
	places AS station
	ON p.pid = station.id
	LEFT JOIN
	places AS line
	ON station.pid = line.id
	LEFT JOIN
	places AS region
	ON line.pid = region.id
	LEFT JOIN
	places AS workshop
	ON region.pid = workshop.id
	LEFT JOIN
	places AS plant
	ON workshop.pid = plant.id
	LEFT JOIN
	places AS root
	ON plant.pid = root.id
	ORDER BY
	d.sn, c.type, c.slot, c.port
END

GO
/****** Object:  StoredProcedure [dbo].[new_channel_status]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[new_channel_status]
	@sn bigint,
	@slot smallint,
	@port smallint,
	@type smallint,
	@ctime datetime,
	@status smallint
AS
BEGIN
	DECLARE @cid bigint
	DECLARE @pid bigint

	SELECT @cid = rs.cid,
	       @pid = rs.pid
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

	UPDATE rs
	SET rs.ctime = @ctime,
	    rs.status = @status
	FROM realtime_status AS rs
	WHERE rs.cid = @cid AND
		  rs.pid = @pid

	INSERT INTO channel_status
	(cid, pid, ctime, status)
	VALUES
	(@cid, @pid, @ctime, @status)
END

GO
/****** Object:  Table [dbo].[calibration]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[calibration](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[sn] [nchar](14) NOT NULL,
	[result] [nvarchar](50) NOT NULL,
	[datetime] [datetime] NOT NULL,
	[expiration] [datetime] NOT NULL,
	[action] [nvarchar](50) NOT NULL,
	[lcl] [nvarchar](50) NULL,
	[ucl] [nvarchar](50) NULL,
	[person] [nvarchar](50) NOT NULL,
 CONSTRAINT [PK_calibration] PRIMARY KEY CLUSTERED
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[channel_changes]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[channel_changes](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[cid] [bigint] NOT NULL,
	[pid] [bigint] NOT NULL,
	[starttime] [datetime] NOT NULL,
	[endtime] [datetime] NULL,
 CONSTRAINT [PK_channel_changes] PRIMARY KEY CLUSTERED
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY],
 CONSTRAINT [IX_channel_changes_u_cid_endtime] UNIQUE NONCLUSTERED
(
	[cid] ASC,
	[endtime] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[channel_data]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[channel_data](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[cid] [bigint] NOT NULL,
	[pid] [bigint] NOT NULL,
	[ctime] [datetime] NOT NULL,
	[data] [float] NOT NULL,
 CONSTRAINT [PK_channel_data] PRIMARY KEY CLUSTERED
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY],
 CONSTRAINT [IX_channel_data_cid_pid_ctime] UNIQUE NONCLUSTERED
(
	[cid] ASC,
	[pid] ASC,
	[ctime] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[channel_status]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[channel_status](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[cid] [bigint] NOT NULL,
	[pid] [bigint] NOT NULL,
	[ctime] [datetime] NOT NULL,
	[status] [smallint] NOT NULL,
 CONSTRAINT [PK_channel_status] PRIMARY KEY CLUSTERED
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY],
 CONSTRAINT [IX_channel_status_cid_pid_ctime] UNIQUE NONCLUSTERED
(
	[cid] ASC,
	[pid] ASC,
	[ctime] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[channels]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[channels](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[did] [bigint] NOT NULL,
	[slot] [smallint] NOT NULL,
	[port] [smallint] NOT NULL,
	[type] [smallint] NOT NULL,
 CONSTRAINT [PK_channels] PRIMARY KEY CLUSTERED
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY],
 CONSTRAINT [IX_channels_u_did_slot_port_type] UNIQUE NONCLUSTERED
(
	[did] ASC,
	[slot] ASC,
	[port] ASC,
	[type] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[devices]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
SET ANSI_PADDING ON
GO
CREATE TABLE [dbo].[devices](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[sn] [bigint] NOT NULL,
	[ip] [varchar](15) NULL,
	[port] [int] NULL,
 CONSTRAINT [PK_devices] PRIMARY KEY CLUSTERED
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
SET ANSI_PADDING OFF
GO
/****** Object:  Table [dbo].[equipments]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[equipments](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[pid] [bigint] NOT NULL,
	[name] [nvarchar](50) NOT NULL,
	[left] [smallint] NOT NULL,
	[top] [smallint] NOT NULL,
	[right] [smallint] NOT NULL,
	[bottom] [smallint] NOT NULL,
 CONSTRAINT [PK_equipments] PRIMARY KEY CLUSTERED
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[failure_processes]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[failure_processes](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[did] [int] NOT NULL,
	[pid] [int] NOT NULL,
	[reason] [nvarchar](50) NOT NULL,
	[solution] [nvarchar](50) NOT NULL,
	[datetime] [datetime] NOT NULL,
	[person] [nvarchar](50) NOT NULL,
 CONSTRAINT [PK_failure_processes] PRIMARY KEY CLUSTERED
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[place_coordinates]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[place_coordinates](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[pid] [bigint] NOT NULL,
	[left] [smallint] NOT NULL,
	[top] [smallint] NOT NULL,
	[right] [smallint] NOT NULL,
	[bottom] [smallint] NOT NULL,
 CONSTRAINT [PK_place_coordinates] PRIMARY KEY CLUSTERED
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY],
 CONSTRAINT [IX_place_coordinates_u_pid] UNIQUE NONCLUSTERED
(
	[pid] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[places]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[places](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[name] [nvarchar](50) NOT NULL,
	[pid] [bigint] NOT NULL,
	[level] [tinyint] NOT NULL,
 CONSTRAINT [PK_places] PRIMARY KEY CLUSTERED
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY],
 CONSTRAINT [IX_places_u_name_pid] UNIQUE NONCLUSTERED
(
	[name] ASC,
	[pid] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[places_users]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[places_users](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[pid] [bigint] NOT NULL,
	[uid] [bigint] NOT NULL,
	[starttime] [datetime] NOT NULL,
	[endtime] [datetime] NULL
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[realtime_status]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[realtime_status](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[cid] [bigint] NOT NULL,
	[pid] [bigint] NOT NULL,
	[ctime] [datetime] NOT NULL,
	[status] [smallint] NOT NULL,
 CONSTRAINT [PK_realtime_status] PRIMARY KEY CLUSTERED
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[roles]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[roles](
	[id] [tinyint] IDENTITY(1,1) NOT NULL,
	[name] [nvarchar](50) NOT NULL,
 CONSTRAINT [PK_roles] PRIMARY KEY CLUSTERED
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[status]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[status](
	[id] [smallint] NOT NULL,
	[name] [nvarchar](50) NOT NULL,
 CONSTRAINT [PK_status] PRIMARY KEY CLUSTERED
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[types]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[types](
	[id] [smallint] IDENTITY(1,1) NOT NULL,
	[name] [nvarchar](50) NOT NULL,
 CONSTRAINT [PK_types] PRIMARY KEY CLUSTERED
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[users]    Script Date: 2017/12/4 14:52:41 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
SET ANSI_PADDING ON
GO
CREATE TABLE [dbo].[users](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[name] [nvarchar](50) NOT NULL,
	[workid] [varchar](50) NOT NULL,
	[password] [varchar](50) NOT NULL,
	[salt] [varchar](50) NOT NULL,
	[rid] [tinyint] NOT NULL,
	[level] [tinyint] NOT NULL,
	[email] [varchar](50) NULL,
 CONSTRAINT [PK_users] PRIMARY KEY CLUSTERED
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
SET ANSI_PADDING OFF
GO
/****** Object:  Index [IX_devices_u_sn]    Script Date: 2017/12/4 14:52:41 ******/
CREATE UNIQUE NONCLUSTERED INDEX [IX_devices_u_sn] ON [dbo].[devices]
(
	[sn] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, SORT_IN_TEMPDB = OFF, IGNORE_DUP_KEY = OFF, DROP_EXISTING = OFF, ONLINE = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
GO
ALTER TABLE [dbo].[users]  WITH CHECK ADD  CONSTRAINT [FK_users_roles] FOREIGN KEY([rid])
REFERENCES [dbo].[roles] ([id])
GO
ALTER TABLE [dbo].[users] CHECK CONSTRAINT [FK_users_roles]
GO
USE [master]
GO
ALTER DATABASE [v2_1_1] SET  READ_WRITE
GO
