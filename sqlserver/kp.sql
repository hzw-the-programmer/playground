USE [master]
GO
/****** Object:  Database [kp]    Script Date: 2018/1/19 16:33:55 ******/
CREATE DATABASE [kp]
 CONTAINMENT = NONE
 ON  PRIMARY 
( NAME = N'kp', FILENAME = N'D:\kp.mdf' , SIZE = 5120KB , MAXSIZE = UNLIMITED, FILEGROWTH = 1024KB )
 LOG ON 
( NAME = N'kp_log', FILENAME = N'D:\kp_log.ldf' , SIZE = 5696KB , MAXSIZE = 2048GB , FILEGROWTH = 10%)
GO
ALTER DATABASE [kp] SET COMPATIBILITY_LEVEL = 110
GO
IF (1 = FULLTEXTSERVICEPROPERTY('IsFullTextInstalled'))
begin
EXEC [kp].[dbo].[sp_fulltext_database] @action = 'enable'
end
GO
ALTER DATABASE [kp] SET ANSI_NULL_DEFAULT OFF 
GO
ALTER DATABASE [kp] SET ANSI_NULLS OFF 
GO
ALTER DATABASE [kp] SET ANSI_PADDING OFF 
GO
ALTER DATABASE [kp] SET ANSI_WARNINGS OFF 
GO
ALTER DATABASE [kp] SET ARITHABORT OFF 
GO
ALTER DATABASE [kp] SET AUTO_CLOSE OFF 
GO
ALTER DATABASE [kp] SET AUTO_CREATE_STATISTICS ON 
GO
ALTER DATABASE [kp] SET AUTO_SHRINK OFF 
GO
ALTER DATABASE [kp] SET AUTO_UPDATE_STATISTICS ON 
GO
ALTER DATABASE [kp] SET CURSOR_CLOSE_ON_COMMIT OFF 
GO
ALTER DATABASE [kp] SET CURSOR_DEFAULT  GLOBAL 
GO
ALTER DATABASE [kp] SET CONCAT_NULL_YIELDS_NULL OFF 
GO
ALTER DATABASE [kp] SET NUMERIC_ROUNDABORT OFF 
GO
ALTER DATABASE [kp] SET QUOTED_IDENTIFIER OFF 
GO
ALTER DATABASE [kp] SET RECURSIVE_TRIGGERS OFF 
GO
ALTER DATABASE [kp] SET  DISABLE_BROKER 
GO
ALTER DATABASE [kp] SET AUTO_UPDATE_STATISTICS_ASYNC OFF 
GO
ALTER DATABASE [kp] SET DATE_CORRELATION_OPTIMIZATION OFF 
GO
ALTER DATABASE [kp] SET TRUSTWORTHY OFF 
GO
ALTER DATABASE [kp] SET ALLOW_SNAPSHOT_ISOLATION OFF 
GO
ALTER DATABASE [kp] SET PARAMETERIZATION SIMPLE 
GO
ALTER DATABASE [kp] SET READ_COMMITTED_SNAPSHOT OFF 
GO
ALTER DATABASE [kp] SET HONOR_BROKER_PRIORITY OFF 
GO
ALTER DATABASE [kp] SET RECOVERY FULL 
GO
ALTER DATABASE [kp] SET  MULTI_USER 
GO
ALTER DATABASE [kp] SET PAGE_VERIFY CHECKSUM  
GO
ALTER DATABASE [kp] SET DB_CHAINING OFF 
GO
ALTER DATABASE [kp] SET FILESTREAM( NON_TRANSACTED_ACCESS = OFF ) 
GO
ALTER DATABASE [kp] SET TARGET_RECOVERY_TIME = 0 SECONDS 
GO
EXEC sys.sp_db_vardecimal_storage_format N'kp', N'ON'
GO
USE [kp]
GO
/****** Object:  StoredProcedure [dbo].[channels_under_station_width_type]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[channels_under_station_width_type]
	@start datetime,
	@end datetime,
	@pid bigint,
	@type smallint
AS
BEGIN
	SELECT
	c.id AS cid, p6.id AS pid
	FROM channel_changes AS cc
	LEFT JOIN channels AS c
	ON cc.cid = c.id
	LEFT JOIN places AS p6
	ON cc.pid = p6.id
	LEFT JOIN places AS p5
	ON p6.pid = p5.id
	LEFT JOIN places AS p4
	ON p5.pid = p4.id
	LEFT JOIN places AS p3
	ON p4.pid = p3.id
	LEFT JOIN places AS p2
	ON p3.pid = p2.id
	LEFT JOIN places AS p1
	ON p2.pid = p1.id
	LEFT JOIN places AS p0
	ON p1.pid = p0.id
	WHERE (
			  (cc.endtime IS NULL AND NOT @end < cc.starttime)
			  OR
			  (cc.endtime IS NOT NULL AND NOT (@start > cc.endtime OR @end < cc.starttime))
		  )
		  AND
		  (
			  p5.id = @pid
		  )
		  AND
		  (
			  c.type = @type
		  )
END








GO
/****** Object:  StoredProcedure [dbo].[clear]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[clear]

AS
BEGIN
	DELETE FROM calibration
	DELETE FROM channel_changes
	DELETE FROM channel_data
	DELETE FROM channel_status
	DELETE FROM channels
	DELETE FROM devices
	DELETE FROM equipments
	DELETE FROM esd_question
	DELETE FROM failures_process
	DELETE FROM monitor_data
	DELETE FROM monitor_obj
	DELETE FROM monitor_pram
	DELETE FROM place_coordinates
	DELETE FROM places
	DELETE FROM places_users
	DELETE FROM pre_status
	DELETE FROM realtime_status
END















GO
/****** Object:  StoredProcedure [dbo].[create_channel]    Script Date: 2018/1/19 16:33:55 ******/
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

	INSERT INTO channel_status
	(cid, pid, ctime, status)
	VALUES
	(@cid, @pid, @datetime, 258)

	INSERT INTO realtime_status
	(cid, pid, ctime, status)
	VALUES
	(@cid, @pid, @datetime, 258)

	INSERT INTO pre_status
	(cid, pid, ctime, status)
	VALUES
	(@cid, @pid, @datetime, 258)



















GO
/****** Object:  StoredProcedure [dbo].[create_place]    Script Date: 2018/1/19 16:33:55 ******/
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
/****** Object:  StoredProcedure [dbo].[delete_channel]    Script Date: 2018/1/19 16:33:55 ******/
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

	INSERT INTO channel_status
	(cid, pid, ctime, status)
	VALUES
	(@cid, @pid, @datetime, 257)

	DELETE FROM realtime_status
	WHERE cid = @cid AND
	      pid = @pid

	DELETE FROM pre_status
	WHERE cid = @cid AND
	      pid = @pid






GO
/****** Object:  StoredProcedure [dbo].[delete_place]    Script Date: 2018/1/19 16:33:55 ******/
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
/****** Object:  StoredProcedure [dbo].[get_channel_at_place_one_day_status]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[get_channel_at_place_one_day_status]
	@cid bigint,
	@pid bigint,
	@date datetime
AS
BEGIN
	DECLARE @start datetime
	DECLARE @end datetime

	SELECT
	@start = MAX(ctime)
	FROM channel_status
	WHERE cid = @cid AND
		  pid = @pid AND
		  ctime < @date

	IF @start IS NULL
		SELECT @start = @date
	SELECT @end = DATEADD(day, 1, @date)

	SELECT
	cs.ctime, cs.status, c.type
	FROM channel_status AS cs
	LEFT JOIN channels AS c
	ON cs.cid = c.id
	WHERE cid = @cid AND
		  pid = @pid AND
		  ctime BETWEEN @start  AND @end
	ORDER BY ctime
END









GO
/****** Object:  StoredProcedure [dbo].[get_channels_under_place]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[get_channels_under_place]
	@pid bigint,
	@page bigint,
	@rows bigint,
	@total bigint OUTPUT
AS
BEGIN
	SElECT
	@total = COUNT(*)
	FROM realtime_status AS rs
	LEFT JOIN channels AS c
	ON rs.cid = c.id
	LEFT JOIN devices AS d
	ON c.did = d.id
	LEFT JOIN types AS t
	ON c.type = t.id
	LEFT JOIN places AS p6
	ON rs.pid = p6.id
	LEFT JOIN places AS p5
	ON p6.pid = p5.id
	LEFT JOIN places AS p4
	ON p5.pid = p4.id
	LEFT JOIN places AS p3
	ON p4.pid = p3.id
	LEFT JOIN places AS p2
	ON p3.pid = p2.id
	LEFT JOIN places AS p1
	ON p2.pid = p1.id
	LEFT JOIN places AS p0
	ON p1.pid = p0.id
	WHERE p0.id = @pid OR
		  p1.id = @pid OR
		  p2.id = @pid OR
		  p3.id = @pid OR
		  p4.id = @pid OR
		  p5.id = @pid OR
		  p6.id = @pid

	SElECT
	d.sn, c.slot, c.port, t.name AS type,
	p1.name AS plant,
	p2.name AS workshop,
	p3.name AS region,
	p4.name AS line,
	p5.name AS station,
	p6.name AS channel
	FROM realtime_status AS rs
	LEFT JOIN channels AS c
	ON rs.cid = c.id
	LEFT JOIN devices AS d
	ON c.did = d.id
	LEFT JOIN types AS t
	ON c.type = t.id
	LEFT JOIN places AS p6
	ON rs.pid = p6.id
	LEFT JOIN places AS p5
	ON p6.pid = p5.id
	LEFT JOIN places AS p4
	ON p5.pid = p4.id
	LEFT JOIN places AS p3
	ON p4.pid = p3.id
	LEFT JOIN places AS p2
	ON p3.pid = p2.id
	LEFT JOIN places AS p1
	ON p2.pid = p1.id
	LEFT JOIN places AS p0
	ON p1.pid = p0.id
	WHERE p0.id = @pid OR
		  p1.id = @pid OR
		  p2.id = @pid OR
		  p3.id = @pid OR
		  p4.id = @pid OR
		  p5.id = @pid OR
		  p6.id = @pid
	ORDER BY d.sn, c.slot, c.port
	OFFSET (@page - 1) * @rows ROWS FETCH NEXT @rows ROWS ONLY
END










GO
/****** Object:  StoredProcedure [dbo].[get_channels_under_station]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[get_channels_under_station]
	@id bigint
AS
BEGIN
	SELECT
	d.id AS did, d.sn, d.ip, d.port,
	c.id AS cid, c.slot, c.port AS cport, t.name AS type,
	p.id AS pid, p.name, p.pid AS ppid, p.level,
	pc.[left], pc.[top], pc.[right], pc.bottom
	FROM realtime_status AS rs
	LEFT JOIN channels AS c
	ON rs.cid = c.id
	LEFT JOIN devices AS d
	ON c.did = d.id
	LEFT JOIN types AS t
	ON c.type = t.id
	LEFT JOIN places AS p
	ON rs.pid = p.id
	LEFT JOIN place_coordinates AS pc
	ON p.id = pc.pid
	WHERE p.pid = @id
	ORDER BY
	d.sn, c.slot, c.port, c.type
END














GO
/****** Object:  StoredProcedure [dbo].[get_devices_under_place]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[get_devices_under_place]
	@pid bigint,
	@page bigint,
	@rows bigint,
	@total bigint OUTPUT
AS
BEGIN
	SELECT
	@total = COUNT(*)
	FROM devices
	WHERE sn IN (
		SELECT
		DISTINCT d.sn
		FROM realtime_status AS rs
		LEFT JOIN channels AS c
		ON rs.cid = c.id
		LEFT JOIN devices AS d
		ON c.did = d.id
		LEFT JOIN places AS p6
		ON rs.pid = p6.id
		LEFT JOIN places AS p5
		ON p6.pid = p5.id
		LEFT JOIN places AS p4
		ON p5.pid = p4.id
		LEFT JOIN places AS p3
		ON p4.pid = p3.id
		LEFT JOIN places AS p2
		ON p3.pid = p2.id
		LEFT JOIN places AS p1
		ON p2.pid = p1.id
		LEFT JOIN places AS p0
		ON p1.pid = p0.id
		WHERE p0.id = @pid OR
			  p1.id = @pid OR
			  p2.id = @pid OR
			  p3.id = @pid OR
			  p4.id = @pid OR
			  p5.id = @pid OR
			  p6.id = @pid
	)

	SELECT
	sn, ip, port
	FROM devices
	WHERE sn IN (
		SELECT
		DISTINCT d.sn
		FROM realtime_status AS rs
		LEFT JOIN channels AS c
		ON rs.cid = c.id
		LEFT JOIN devices AS d
		ON c.did = d.id
		LEFT JOIN places AS p6
		ON rs.pid = p6.id
		LEFT JOIN places AS p5
		ON p6.pid = p5.id
		LEFT JOIN places AS p4
		ON p5.pid = p4.id
		LEFT JOIN places AS p3
		ON p4.pid = p3.id
		LEFT JOIN places AS p2
		ON p3.pid = p2.id
		LEFT JOIN places AS p1
		ON p2.pid = p1.id
		LEFT JOIN places AS p0
		ON p1.pid = p0.id
		WHERE p0.id = @pid OR
			  p1.id = @pid OR
			  p2.id = @pid OR
			  p3.id = @pid OR
			  p4.id = @pid OR
			  p5.id = @pid OR
			  p6.id = @pid
	)
	ORDER BY sn
	OFFSET (@page - 1) * @rows ROWS FETCH NEXT @rows ROWS ONLY
END










GO
/****** Object:  StoredProcedure [dbo].[get_on_channels]    Script Date: 2018/1/19 16:33:55 ******/
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
	station.id AS stationid,
	CASE rs.status
	WHEN 256 THEN ps.status ELSE rs.status END AS prestatus,
	CASE rs.status
	WHEN 256 THEN ps.ctime ELSE rs.ctime END AS prectime
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
	LEFT JOIN pre_status AS ps
	ON rs.cid = ps.cid AND rs.pid = ps.pid
	ORDER BY
	d.sn, c.slot, c.port, c.type
END















GO
/****** Object:  StoredProcedure [dbo].[get_realtime_status]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[get_realtime_status]
AS
BEGIN
	SELECT
	d.sn,
	c.slot, c.port AS cport, t.name AS type,
	CONVERT(char, rs.ctime, 120) AS time,
	rs.status,
	p0.id AS rootid,
	p1.id AS plantid,
	p2.id AS workshopid,
	p3.id AS regionid,
	p4.id AS lineid,
	p5.id AS stationid,
	p6.id AS channelid
	FROM realtime_status AS rs
	LEFT JOIN channels AS c
	ON rs.cid = c.id
	LEFT JOIN devices AS d
	ON c.did = d.id
	LEFT JOIN types AS t
	ON c.type = t.id
	LEFT JOIN places AS p6
	ON rs.pid = p6.id
	LEFT JOIN places AS p5
	ON p6.pid = p5.id
	LEFT JOIN places AS p4
	ON p5.pid = p4.id
	LEFT JOIN places AS p3
	ON p4.pid = p3.id
	LEFT JOIN places AS p2
	ON p3.pid = p2.id
	LEFT JOIN places AS p1
	ON p2.pid = p1.id
	LEFT JOIN places AS p0
	ON p1.pid = p0.id
	ORDER BY d.sn, c.slot
END














GO
/****** Object:  StoredProcedure [dbo].[get_stations_under_line_one_day]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[get_stations_under_line_one_day]
	@pid bigint,
	@date datetime,
	@total bigint OUTPUT,
	@page bigint,
	@rows bigint
AS
BEGIN
	DECLARE @start datetime
	DECLARE @end datetime

	SELECT @start = @date
	SELECT @end = DATEADD(day, 1, @start)

	SELECT
	@total = COUNT(*)
	FROM (
		SELECT
		p1.name AS plant,
		p2.name AS workshop,
		p3.name AS region,
		p4.name AS line,
		p5.name AS station,
		t.name AS type,
		p5.id AS pid, c.type AS tid
		FROM channel_status AS cs
		LEFT JOIN channels AS c
		ON cs.cid = c.id
		LEFT JOIN types AS t
		ON c.type = t.id
		LEFT JOIN places AS p6
		ON cs.pid = p6.id
		LEFT JOIN places AS p5
		ON p6.pid = p5.id
		LEFT JOIN places AS p4
		ON p5.pid = p4.id
		LEFT JOIN places AS p3
		ON p4.pid = p3.id
		LEFT JOIN places AS p2
		ON p3.pid = p2.id
		LEFT JOIN places AS p1
		ON p2.pid = p1.id
		WHERE cs.ctime BETWEEN @start AND @end AND
			  p4.id = @pid
		GROUP BY p1.name, p2.name, p3.name, p4.name, p5.id, p5.name, c.type, t.name
	) AS temp

	SELECT
	p1.name AS plant,
	p2.name AS workshop,
	p3.name AS region,
	p4.name AS line,
	p5.name AS station,
	t.name AS type,
	p5.id AS pid, c.type AS tid
	FROM channel_status AS cs
	LEFT JOIN channels AS c
	ON cs.cid = c.id
	LEFT JOIN types AS t
	ON c.type = t.id
	LEFT JOIN places AS p6
	ON cs.pid = p6.id
	LEFT JOIN places AS p5
	ON p6.pid = p5.id
	LEFT JOIN places AS p4
	ON p5.pid = p4.id
	LEFT JOIN places AS p3
	ON p4.pid = p3.id
	LEFT JOIN places AS p2
	ON p3.pid = p2.id
	LEFT JOIN places AS p1
	ON p2.pid = p1.id
	WHERE cs.ctime BETWEEN @start AND @end AND
		  p4.id = @pid
	GROUP BY p1.name, p2.name, p3.name, p4.name, p5.id, p5.name, c.type, t.name
	ORDER BY plant, workshop, region, line, station, t.name
	OFFSET (@page - 1) * @rows ROWS FETCH NEXT @rows ROWS ONLY
END









GO
/****** Object:  StoredProcedure [dbo].[group_dates_types_under_station]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[group_dates_types_under_station]
	@start datetime,
	@end datetime,
	@pid bigint,
	@page bigint,
	@rows bigint,
	@total bigint OUTPUT
AS
BEGIN
	SELECT
	@total = COUNT(*)
	FROM (
		SELECT
		*
		FROM (
			SELECT
			p0.name AS root,
			p1.name AS plant,
			p2.name AS workshop,
			p3.name AS region,
			p4.name AS line,
			p5.name AS station,
			t.name AS type,
			p5.id AS pid, c.type AS tid,
			CONVERT(date, cc.starttime, 120) AS date
			FROM channel_changes AS cc
			LEFT JOIN channels AS c
			ON cc.cid = c.id
			LEFT JOIN types AS t
			ON c.type = t.id
			LEFT JOIN places AS p6
			ON cc.pid = p6.id
			LEFT JOIN places AS p5
			ON p6.pid = p5.id
			LEFT JOIN places AS p4
			ON p5.pid = p4.id
			LEFT JOIN places AS p3
			ON p4.pid = p3.id
			LEFT JOIN places AS p2
			ON p3.pid = p2.id
			LEFT JOIN places AS p1
			ON p2.pid = p1.id
			LEFT JOIN places AS p0
			ON p1.pid = p0.id
			WHERE (
						(cc.endtime IS NULL AND NOT @end < cc.starttime)
						OR
						(cc.endtime IS NOT NULL AND NOT (@start > cc.endtime OR @end < cc.starttime))
					)
					AND
					(
						p5.id = @pid
					)
		) temp
		GROUP BY date, root, plant, workshop, region, line, pid, station, tid, type
	) AS temp

	IF @page = 0
	BEGIN
		SELECT @page = 1
		IF @total != 0
			SELECT @rows = @total
	END

	SELECT
	*
	FROM (
		SELECT
		p0.name AS root,
		p1.name AS plant,
		p2.name AS workshop,
		p3.name AS region,
		p4.name AS line,
		p5.name AS station,
		t.name AS type,
		p5.id AS pid, c.type AS tid,
		CONVERT(date, cc.starttime, 120) AS date
		FROM channel_changes AS cc
		LEFT JOIN channels AS c
		ON cc.cid = c.id
		LEFT JOIN types AS t
		ON c.type = t.id
		LEFT JOIN places AS p6
		ON cc.pid = p6.id
		LEFT JOIN places AS p5
		ON p6.pid = p5.id
		LEFT JOIN places AS p4
		ON p5.pid = p4.id
		LEFT JOIN places AS p3
		ON p4.pid = p3.id
		LEFT JOIN places AS p2
		ON p3.pid = p2.id
		LEFT JOIN places AS p1
		ON p2.pid = p1.id
		LEFT JOIN places AS p0
		ON p1.pid = p0.id
		WHERE (
					(cc.endtime IS NULL AND NOT @end < cc.starttime)
					OR
					(cc.endtime IS NOT NULL AND NOT (@start > cc.endtime OR @end < cc.starttime))
				)
				AND
				(
					p5.id = @pid
				)
	) AS temp
	GROUP BY date, root, plant, workshop, region, line, pid, station, tid, type
	ORDER BY date, root, plant, workshop, region, line, station, type
	OFFSET (@page - 1) * @rows ROWS FETCH NEXT @rows ROWS ONLY
END








GO
/****** Object:  StoredProcedure [dbo].[group_stations_types_under_line]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[group_stations_types_under_line]
	@start datetime,
	@end datetime,
	@pid bigint,
	@page bigint,
	@rows bigint,
	@total bigint OUTPUT
AS
BEGIN
	SELECT
	@total = COUNT(*)
	FROM (
		SELECT
		p0.name AS root,
		p1.name AS plant,
		p2.name AS workshop,
		p3.name AS region,
		p4.name AS line,
		p5.name AS station,
		t.name AS type,
		p5.id AS pid, c.type AS tid
		FROM channel_changes AS cc
		LEFT JOIN channels AS c
		ON cc.cid = c.id
		LEFT JOIN types AS t
		ON c.type = t.id
		LEFT JOIN places AS p6
		ON cc.pid = p6.id
		LEFT JOIN places AS p5
		ON p6.pid = p5.id
		LEFT JOIN places AS p4
		ON p5.pid = p4.id
		LEFT JOIN places AS p3
		ON p4.pid = p3.id
		LEFT JOIN places AS p2
		ON p3.pid = p2.id
		LEFT JOIN places AS p1
		ON p2.pid = p1.id
		LEFT JOIN places AS p0
		ON p1.pid = p0.id
		WHERE (
				  (cc.endtime IS NULL AND NOT @end < cc.starttime)
				  OR
				  (cc.endtime IS NOT NULL AND NOT (@start > cc.endtime OR @end < cc.starttime))
			  )
			  AND
			  (
				  p4.id = @pid
			  )
		GROUP BY p0.name, p1.name, p2.name, p3.name, p4.name, p5.id, p5.name, c.type, t.name
	) AS temp

	IF @page = 0
	BEGIN
		SELECT @page = 1
		IF @total != 0
			SELECT @rows = @total
	END

	SELECT
	p0.name AS root,
	p1.name AS plant,
	p2.name AS workshop,
	p3.name AS region,
	p4.name AS line,
	p5.name AS station,
	t.name AS type,
	p5.id AS pid, c.type AS tid
	FROM channel_changes AS cc
	LEFT JOIN channels AS c
	ON cc.cid = c.id
	LEFT JOIN types AS t
	ON c.type = t.id
	LEFT JOIN places AS p6
	ON cc.pid = p6.id
	LEFT JOIN places AS p5
	ON p6.pid = p5.id
	LEFT JOIN places AS p4
	ON p5.pid = p4.id
	LEFT JOIN places AS p3
	ON p4.pid = p3.id
	LEFT JOIN places AS p2
	ON p3.pid = p2.id
	LEFT JOIN places AS p1
	ON p2.pid = p1.id
	LEFT JOIN places AS p0
	ON p1.pid = p0.id
	WHERE (
				(cc.endtime IS NULL AND NOT @end < cc.starttime)
				OR
				(cc.endtime IS NOT NULL AND NOT (@start > cc.endtime OR @end < cc.starttime))
			)
			AND
			(
				p4.id = @pid
			)
	GROUP BY p0.name, p1.name, p2.name, p3.name, p4.name, p5.id, p5.name, c.type, t.name
	ORDER BY p0.name, p1.name, p2.name, p3.name, p4.name, p5.name, t.name
	OFFSET (@page - 1) * @rows ROWS FETCH NEXT @rows ROWS ONLY
END








GO
/****** Object:  StoredProcedure [dbo].[new_channel_data]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[new_channel_data]
	@sn bigint, 
	@slot smallint,
	@port smallint,
	@type smallint,
	@ctime datetime,
	@data float
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

	INSERT INTO channel_data
	(cid, pid, ctime, data)
	VALUES
	(@cid, @pid, @ctime, @data)
END














GO
/****** Object:  StoredProcedure [dbo].[new_channel_status]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[new_channel_status]
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

	INSERT INTO channel_status
	(cid, pid, ctime, status)
	VALUES
	(@cid, @pid, @nctime, @nstatus)

	IF @nstatus <= 255
	BEGIN
		IF (@type = 9 AND (@nstatus & 0xE0) = 0x20) OR (@type != 9 AND (@nstatus & 0x20) = 0x20)
		BEGIN
			INSERT INTO failures_process
			(cid, pid, ctime, status, solved)
			VALUES
			(@cid, @pid, @nctime, @nstatus, 0)
		END
	END

	IF @nstatus = 256 --OFFLINE
	BEGIN
		UPDATE pre_status
		SET ctime = @cctime,
			status = @cstatus
		WHERE cid = @cid AND
			  pid = @pid

		UPDATE realtime_status
		SET ctime = @nctime,
			status = @nstatus
		WHERE cid = @cid AND
              pid = @pid

		RETURN 1
	END

	IF @nstatus = 259 --ONLINE
	BEGIN
		UPDATE realtime_status
		SET ctime = @pctime,
		    status = @pstatus
		WHERE cid = @cid AND
              pid = @pid

		RETURN 1
	END

	IF @nstatus = 260 --POWERON
		RETURN 1

	IF @nctime > @cctime
	BEGIN
		UPDATE realtime_status
		SET ctime = @nctime,
			status = @nstatus
		WHERE cid = @cid AND
			  pid = @pid
	END
END















GO
/****** Object:  StoredProcedure [dbo].[t2]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[t2]
	@start datetime,
	@end datetime,
	@pid bigint,
	@type smallint
AS
BEGIN
	SELECT
	p0.name AS root,
	p1.name AS plant,
	p2.name AS workshop,
	p3.name AS region,
	p4.name AS line,
	p5.name AS station,
	p6.name AS channel,
	c.id AS cid, c.slot, c.port, t.name AS type,
	cc.starttime, cc.endtime
	FROM channel_changes AS cc
	LEFT JOIN channels AS c
	ON cc.cid = c.id
	LEFT JOIN types AS t
	ON c.type = t.id
	LEFT JOIN places AS p6
	ON cc.pid = p6.id
	LEFT JOIN places AS p5
	ON p6.pid = p5.id
	LEFT JOIN places AS p4
	ON p5.pid = p4.id
	LEFT JOIN places AS p3
	ON p4.pid = p3.id
	LEFT JOIN places AS p2
	ON p3.pid = p2.id
	LEFT JOIN places AS p1
	ON p2.pid = p1.id
	LEFT JOIN places AS p0
	ON p1.pid = p0.id
	WHERE (
			  (cc.endtime IS NULL AND NOT @end < cc.starttime)
			  OR
			  (cc.endtime IS NOT NULL AND NOT (@start > cc.endtime OR @end < cc.starttime))
		  )
		  AND
		  (
			  p0.id = @pid OR
			  p1.id = @pid OR
			  p2.id = @pid OR
			  p3.id = @pid OR
			  p4.id = @pid OR
			  p5.id = @pid OR
			  p6.id = @pid
		  )
		  AND
		  (
			  c.type = @type
		  )
END








GO
/****** Object:  StoredProcedure [dbo].[update_device_ta]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE PROCEDURE [dbo].[update_device_ta]
	@sn bigint,
	@ip varchar(15),
	@port int,
	@deveui varchar(20)
AS
BEGIN
	UPDATE d
	SET d.ip = '',
	    d.port = 0
	FROM devices AS d
	WHERE d.ip = @ip AND d.port = @port

	UPDATE d
	SET d.deveui = ''
	FROM devices AS d
	WHERE d.deveui = @deveui

	DECLARE @id bigint
	SELECT @id = id FROM devices WHERE sn = @sn
	IF @id IS NULL
	BEGIN
		INSERT INTO devices
		(sn, ip, port, deveui)
		VALUES
		(@sn, @ip, @port, @deveui)
	END
	ELSE
	BEGIN
		UPDATE d
		SET d.ip = @ip,
		    d.port = @port,
			d.deveui = @deveui
		FROM devices AS d
		WHERE d.id = @id
	END
END


GO
/****** Object:  Table [dbo].[calibration]    Script Date: 2018/1/19 16:33:55 ******/
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
/****** Object:  Table [dbo].[channel_changes]    Script Date: 2018/1/19 16:33:55 ******/
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
/****** Object:  Table [dbo].[channel_data]    Script Date: 2018/1/19 16:33:55 ******/
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
/****** Object:  Table [dbo].[channel_status]    Script Date: 2018/1/19 16:33:55 ******/
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
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[channels]    Script Date: 2018/1/19 16:33:55 ******/
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
/****** Object:  Table [dbo].[devices]    Script Date: 2018/1/19 16:33:55 ******/
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
	[deveui] [varchar](20) NULL,
 CONSTRAINT [PK_devices] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
SET ANSI_PADDING OFF
GO
/****** Object:  Table [dbo].[equipments]    Script Date: 2018/1/19 16:33:55 ******/
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
/****** Object:  Table [dbo].[esd_question]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[esd_question](
	[question_id] [int] IDENTITY(1,1) NOT NULL,
	[datetime] [datetime] NOT NULL,
	[line_area] [nvarchar](50) NOT NULL,
	[question_desc] [text] NULL,
	[reason] [text] NULL,
	[action] [text] NULL,
	[responsible] [nvarchar](50) NULL,
	[action_time] [datetime] NOT NULL,
	[status] [nvarchar](50) NOT NULL,
	[scope] [nvarchar](50) NOT NULL,
 CONSTRAINT [PK_esd_question] PRIMARY KEY CLUSTERED 
(
	[question_id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY] TEXTIMAGE_ON [PRIMARY]

GO
/****** Object:  Table [dbo].[failures_process]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[failures_process](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[cid] [bigint] NOT NULL,
	[pid] [bigint] NOT NULL,
	[ctime] [datetime] NOT NULL,
	[status] [smallint] NOT NULL,
	[solved] [bit] NOT NULL,
	[cause] [nvarchar](50) NULL,
	[solution] [nvarchar](50) NULL,
	[stime] [datetime] NULL,
	[uid] [bigint] NULL,
 CONSTRAINT [PK_failures] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[monitor_data]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[monitor_data](
	[obj_id] [int] NOT NULL,
	[sam_id] [nvarchar](50) NOT NULL,
	[pram_id] [nvarchar](max) NOT NULL,
	[pram_value] [numeric](18, 0) NOT NULL,
	[unit] [nvarchar](10) NULL,
	[status] [nvarchar](50) NULL,
	[esd_type] [int] NOT NULL,
	[checker] [nvarchar](50) NULL,
	[checktime] [datetime] NOT NULL,
	[plant] [nvarchar](50) NULL,
	[workshop] [nvarchar](50) NULL,
	[region] [nvarchar](50) NULL,
	[line] [nvarchar](50) NULL,
	[station] [nvarchar](50) NULL,
	[port] [nvarchar](50) NULL,
	[notes] [nvarchar](128) NULL
) ON [PRIMARY] TEXTIMAGE_ON [PRIMARY]

GO
/****** Object:  Table [dbo].[monitor_obj]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
SET ANSI_PADDING ON
GO
CREATE TABLE [dbo].[monitor_obj](
	[obj_id] [int] IDENTITY(1,1) NOT NULL,
	[obj_name] [varchar](50) NOT NULL,
	[obj_cpram] [varchar](256) NULL,
	[obj_pid] [int] NULL,
	[obj_did] [int] NULL,
 CONSTRAINT [PK_monitor_obj] PRIMARY KEY CLUSTERED 
(
	[obj_id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
SET ANSI_PADDING OFF
GO
/****** Object:  Table [dbo].[monitor_pram]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[monitor_pram](
	[id] [nvarchar](50) NOT NULL,
	[name] [nvarchar](50) NOT NULL,
	[unit] [nvarchar](20) NOT NULL,
	[ucl] [numeric](18, 0) NOT NULL,
	[lcl] [numeric](18, 0) NOT NULL,
	[fitobj] [nvarchar](60) NULL
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[place_coordinates]    Script Date: 2018/1/19 16:33:55 ******/
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
/****** Object:  Table [dbo].[places]    Script Date: 2018/1/19 16:33:55 ******/
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
/****** Object:  Table [dbo].[places_users]    Script Date: 2018/1/19 16:33:55 ******/
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
/****** Object:  Table [dbo].[pre_status]    Script Date: 2018/1/19 16:33:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[pre_status](
	[id] [bigint] IDENTITY(1,1) NOT NULL,
	[cid] [bigint] NOT NULL,
	[pid] [bigint] NOT NULL,
	[ctime] [datetime] NOT NULL,
	[status] [smallint] NOT NULL,
 CONSTRAINT [PK_pre_status] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[realtime_status]    Script Date: 2018/1/19 16:33:55 ******/
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
/****** Object:  Table [dbo].[roles]    Script Date: 2018/1/19 16:33:55 ******/
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
/****** Object:  Table [dbo].[status]    Script Date: 2018/1/19 16:33:55 ******/
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
/****** Object:  Table [dbo].[types]    Script Date: 2018/1/19 16:33:55 ******/
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
/****** Object:  Table [dbo].[users]    Script Date: 2018/1/19 16:33:55 ******/
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
/****** Object:  Index [IX_devices_u_sn]    Script Date: 2018/1/19 16:33:55 ******/
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
ALTER DATABASE [kp] SET  READ_WRITE 
GO
