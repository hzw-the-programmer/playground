USE [master]
GO
/****** Object:  Database [iotdb]    Script Date: 2018/11/8 13:59:18 ******/
CREATE DATABASE [iotdb]
 CONTAINMENT = NONE
 ON  PRIMARY 
( NAME = N'iotdb', FILENAME = N'D:\iotdb.mdf' , SIZE = 19456KB , MAXSIZE = UNLIMITED, FILEGROWTH = 1024KB )
 LOG ON 
( NAME = N'iotdb_log', FILENAME = N'D:\iotdb_log.ldf' , SIZE = 1341696KB , MAXSIZE = 2048GB , FILEGROWTH = 10%)
GO
ALTER DATABASE [iotdb] SET COMPATIBILITY_LEVEL = 110
GO
IF (1 = FULLTEXTSERVICEPROPERTY('IsFullTextInstalled'))
begin
EXEC [iotdb].[dbo].[sp_fulltext_database] @action = 'enable'
end
GO
ALTER DATABASE [iotdb] SET ANSI_NULL_DEFAULT OFF 
GO
ALTER DATABASE [iotdb] SET ANSI_NULLS OFF 
GO
ALTER DATABASE [iotdb] SET ANSI_PADDING OFF 
GO
ALTER DATABASE [iotdb] SET ANSI_WARNINGS OFF 
GO
ALTER DATABASE [iotdb] SET ARITHABORT OFF 
GO
ALTER DATABASE [iotdb] SET AUTO_CLOSE OFF 
GO
ALTER DATABASE [iotdb] SET AUTO_CREATE_STATISTICS ON 
GO
ALTER DATABASE [iotdb] SET AUTO_SHRINK OFF 
GO
ALTER DATABASE [iotdb] SET AUTO_UPDATE_STATISTICS ON 
GO
ALTER DATABASE [iotdb] SET CURSOR_CLOSE_ON_COMMIT OFF 
GO
ALTER DATABASE [iotdb] SET CURSOR_DEFAULT  GLOBAL 
GO
ALTER DATABASE [iotdb] SET CONCAT_NULL_YIELDS_NULL OFF 
GO
ALTER DATABASE [iotdb] SET NUMERIC_ROUNDABORT OFF 
GO
ALTER DATABASE [iotdb] SET QUOTED_IDENTIFIER OFF 
GO
ALTER DATABASE [iotdb] SET RECURSIVE_TRIGGERS OFF 
GO
ALTER DATABASE [iotdb] SET  DISABLE_BROKER 
GO
ALTER DATABASE [iotdb] SET AUTO_UPDATE_STATISTICS_ASYNC OFF 
GO
ALTER DATABASE [iotdb] SET DATE_CORRELATION_OPTIMIZATION OFF 
GO
ALTER DATABASE [iotdb] SET TRUSTWORTHY OFF 
GO
ALTER DATABASE [iotdb] SET ALLOW_SNAPSHOT_ISOLATION OFF 
GO
ALTER DATABASE [iotdb] SET PARAMETERIZATION SIMPLE 
GO
ALTER DATABASE [iotdb] SET READ_COMMITTED_SNAPSHOT OFF 
GO
ALTER DATABASE [iotdb] SET HONOR_BROKER_PRIORITY OFF 
GO
ALTER DATABASE [iotdb] SET RECOVERY FULL 
GO
ALTER DATABASE [iotdb] SET  MULTI_USER 
GO
ALTER DATABASE [iotdb] SET PAGE_VERIFY CHECKSUM  
GO
ALTER DATABASE [iotdb] SET DB_CHAINING OFF 
GO
ALTER DATABASE [iotdb] SET FILESTREAM( NON_TRANSACTED_ACCESS = OFF ) 
GO
ALTER DATABASE [iotdb] SET TARGET_RECOVERY_TIME = 0 SECONDS 
GO
EXEC sys.sp_db_vardecimal_storage_format N'iotdb', N'ON'
GO
USE [iotdb]
GO
/****** Object:  StoredProcedure [dbo].[clear]    Script Date: 2018/11/8 13:59:18 ******/
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
	DELETE FROM device_rssi
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
/****** Object:  StoredProcedure [dbo].[closeMpoint]    Script Date: 2018/11/8 13:59:18 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-11-07
-- Description:	closeMPoint
-- =============================================
CREATE PROCEDURE [dbo].[closeMpoint]
	@mid INT,
	@dt BIGINT
AS
BEGIN
	SET NOCOUNT ON;

	DECLARE @OFF INT = 257;

	DELETE FROM mpoint_realtime_status WHERE mpoint_id = @mid;
	
	DELETE FROM mpoint_pre_status WHERE mpoint_id = @mid;
	
	INSERT INTO mpoint_status (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
	VALUES (@mid, 0, @OFF, @OFF, 0, @dt, @dt);

	UPDATE mpoint
	SET endtime = @dt
	WHERE id = @mid
END

GO
/****** Object:  StoredProcedure [dbo].[createVirtualDevices]    Script Date: 2018/11/8 13:59:18 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO


-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-08-08
-- Description:	createVirtualDevices
-- =============================================
CREATE PROCEDURE [dbo].[createVirtualDevices]
	@lineCount INT,
	@stationPerLine INT
AS
BEGIN
	-- SET NOCOUNT ON added to prevent extra result sets from
	-- interfering with SELECT statements.
	SET NOCOUNT ON;

	DECLARE @GND_H INT = 8
	DECLARE @GND_L INT = 12
	DECLARE @WS INT = 9

	DECLARE @sn BIGINT = 20180123000001

	DECLARE @dt BIGINT = DATEDIFF(SECOND, '1970-01-01 08:00:00', GETDATE())

	--
	-- Factory
	--
	DECLARE @factoryName NVARCHAR(20) = N'���⹤��'

	DECLARE @factoryId INT
	SELECT @factoryId = id FROM place WHERE name = @factoryName AND level = 1

	IF @factoryId IS NULL
	BEGIN
		INSERT INTO place (name, pid, level, del)
		VALUES (@factoryName, 0, 1, 0)

		SET @factoryId = SCOPE_IDENTITY()
	END

	--
	-- Workshop
	--
	DECLARE @workshopName NVARCHAR(20) = N'���⳵��'

	DECLARE @workshopId INT
	SELECT @workshopId = id FROM place WHERE name = @workshopName AND level = 2

	IF @workshopId IS NULL
	BEGIN
		INSERT INTO place (name, pid, level, del)
		VALUES (@workshopName, @factoryId, 2, 0)

		SET @workshopId = SCOPE_IDENTITY()
	END

	--
	-- Region
	--
	DECLARE @regionName NVARCHAR(20) = N'��������'

	DECLARE @regionId INT
	SELECT @regionId = id FROM place WHERE name = @regionName AND level = 3

	IF @regionId IS NULL
	BEGIN
		INSERT INTO place (name, pid, level, del)
		VALUES (@regionName, @workshopId, 3, 0)

		SET @regionId = SCOPE_IDENTITY()
	END

	--
	-- Line
	--
	DECLARE @lineName NVARCHAR(20) = N'��������'

	DECLARE @lineNum INT = 0

	WHILE @lineNum < @lineCount
	BEGIN
		DECLARE @lineId INT = NULL

		SELECT @lineId = id FROM place WHERE name = CONCAT(@lineName, @lineNum) AND level = 4

		IF @lineId IS NULL
		BEGIN
			INSERT INTO place (name, pid, level, del)
			VALUES (CONCAT(@lineName, @lineNum), @regionId, 4, 0)

			SET @lineId = SCOPE_IDENTITY()
		END

		--
		-- Station
		--
		DECLARE @stationName NVARCHAR(20) = N'���⹤λ'
		DECLARE @stationNum INT = 0

		WHILE @stationNum < @stationPerLine
		BEGIN
			DECLARE @stationId INT = NULL

			SELECT @stationId = id
			FROM place
			WHERE name = CONCAT(@stationName, @stationNum) AND level = 5 AND pid = @lineId
			
			IF @stationId IS NULL
			BEGIN
				INSERT INTO place (name, pid, level, del)
				VALUES (CONCAT(@stationName, @stationNum), @lineId, 5, 0)

				SET @stationId = SCOPE_IDENTITY()
			END

			---
			--- Device
			---
			DECLARE @deviceId INT = NULL
			SELECT @deviceId = id FROM devices_info WHERE sn = @sn

			IF @deviceId IS NULL
			BEGIN
				INSERT INTO devices_info (sn, ip, type, status, parent_id, level, createtime, updatetime)
				VALUES (@sn, '127.0.0.1', 'KEDAS', 0, 0, 1, @dt, @dt)

				SET @deviceId = SCOPE_IDENTITY()
			END

			DECLARE @slot INT = 1

			WHILE @slot <= 4
			BEGIN
				DECLARE @port INT = 0

				WHILE @port < 4
				BEGIN
					DECLARE @type INT = NULL

					IF @slot = 1 OR @slot = 2
					BEGIN
						SET @type = @GND_H
					END
					ELSE IF @slot = 3
					BEGIN
						SET @type = @GND_L
					END
					ELSE IF @port = 0 OR @port = 1
					BEGIN
						SET @type = @WS
					END

					IF @type IS NOT NULL
					BEGIN
						DECLARE @name NVARCHAR(20) = CONCAT(@slot, @port)
						EXEC new_mpoint @sn, @slot, @port, @type, @stationId, @name, @dt
					END

					SET @port = @port + 1
				END

				SET @slot = @slot + 1
			END

			SET @sn = @sn + 1

			SET @stationNum = @stationNum + 1
		END

		SET @lineNum = @lineNum + 1
	END
END




GO
/****** Object:  StoredProcedure [dbo].[delete_mpoint]    Script Date: 2018/11/8 13:59:18 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO


-- =============================================
-- Author:		wujun
-- Create date: 2018-06-26
-- Description:	���� SN ɾ����ص�-��ɾ��realtime_status->pre_status->mpoint�����endtime
-- Update date: 2018-07-17
-- Description:	��������off״̬
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
	  AND mp.endtime = 0

END





GO
/****** Object:  StoredProcedure [dbo].[delete_mpoint_one]    Script Date: 2018/11/8 13:59:18 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO


-- =============================================
-- Author:		wujun
-- Create date: 2018-06-26
-- Description:	���� MID ɾ����ص�-��ɾ��realtime_status->pre_status->mpoint�����endtime
-- =============================================
CREATE PROCEDURE [dbo].[delete_mpoint_one]
 @id int

AS
BEGIN

	DELETE mrs
	FROM mpoint_realtime_status AS mrs
	WHERE mrs.mpoint_id = @id

	DELETE mps
	FROM mpoint_pre_status AS mps
	WHERE mps.mpoint_id = @id

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
/****** Object:  StoredProcedure [dbo].[deviceSignal]    Script Date: 2018/11/8 13:59:18 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO

-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-08-07
-- Description:	deviceSignal
-- =============================================
CREATE PROCEDURE [dbo].[deviceSignal]
	@sn NVARCHAR(20),
	@signal SMALLINT,
	@dt BIGINT
AS
BEGIN
	-- SET NOCOUNT ON added to prevent extra result sets from
	-- interfering with SELECT statements.
	SET NOCOUNT ON;

	--DECLARE @now BIGINT
	--SELECT @now = DATEDIFF(SECOND, '1970-01-01 08:00:00', GETDATE())

	UPDATE ds
	SET ds.rssi = @signal,
	    ds.updatetime = @dt
	FROM device_rssi AS ds
	LEFT JOIN devices_info AS di
	ON ds.device_id = di.id
	WHERE di.sn = @sn
END



GO
/****** Object:  StoredProcedure [dbo].[insert_delete_status]    Script Date: 2018/11/8 13:59:18 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
-- =============================================
-- Author:		WUJUN
-- Create date: 2018-07-17
-- Description:	����SN��������OFF״̬
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
	  AND mp.endtime = 0

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
/****** Object:  StoredProcedure [dbo].[new_channel_data]    Script Date: 2018/11/8 13:59:18 ******/
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
	@dt BIGINT,
	@seq BIGINT,
	@pcdt BIGINT,
	@data float
AS
BEGIN
	SET NOCOUNT ON

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

	--DECLARE @count int

	--SELECT @count = COUNT(*)
	--FROM mpoint_data
	--WHERE mpoint_id = @mpid AND
	--      time BETWEEN @dt - 30 AND @dt + 30

	--EXEC updateOfflineTime @sn, @slot, @port, @type, @dt

	--IF @count = 0
	--BEGIN
		INSERT INTO mpoint_data (mpoint_id, swiftnum, data, time, createtime)
		VALUES (@mpid, @seq, @data, @dt, @pcdt)
	--END
END






GO
/****** Object:  StoredProcedure [dbo].[new_channel_realtime_status]    Script Date: 2018/11/8 13:59:18 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO

-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-08-11
-- Description:	new_channel_realtime_status
-- =============================================
CREATE PROCEDURE [dbo].[new_channel_realtime_status]
	@sn NVARCHAR(20),
	@slot TINYINT,
	@port TINYINT,
	@type TINYINT,
	@dt BIGINT,
	@seq BIGINT,
	@pcdt BIGINT,
	@status SMALLINT
AS
BEGIN
	SET NOCOUNT ON
	
	DECLARE @GND_H TINYINT = 8
	DECLARE @GND_L TINYINT = 12
	DECLARE @WS TINYINT = 9

	DECLARE @OFF SMALLINT = 257
	DECLARE @ON SMALLINT = 258
	DECLARE @OFFLINE SMALLINT = 256
	DECLARE @ONLINE SMALLINT = 259
	DECLARE @POWEROFF SMALLINT = 261
	DECLARE @POWERON SMALLINT = 260

    --
	--  check mpoint exists
	--
	DECLARE @mpid INT = NULL

	SELECT @mpid = mp.id
	FROM mpoint AS mp
	LEFT JOIN channels_info AS ci
	ON mp.ciid = ci.id
	LEFT JOIN devices_info AS di
	ON ci.device_id = di.id
	WHERE di.sn = @sn
	  AND ci.slot = @slot
	  AND ci.port = @port
	  AND ci.type = @type
	  AND mp.endtime = 0

	IF @mpid IS NULL
	BEGIN
		RETURN
	END

	--
	-- process status
	--
	DECLARE @real_status SMALLINT, @level SMALLINT

	SELECT @real_status = CASE
		WHEN @status < 256 THEN
		CASE
			WHEN @type = @WS THEN
			CASE
				WHEN @status & 0x80 = 0x80 THEN 0x80
				WHEN @status & 0x50 = 0x50 THEN 0x50
				WHEN @status & 0x40 = 0x40 THEN 0x40
				WHEN @status & 0x20 = 0x20 THEN 0x20
				ELSE 0x00
			END
			ELSE
			CASE
				WHEN @status & 0x20 = 0x20 THEN 0x20
				ELSE 0x00
			END
		END
		ELSE @status
	END

	SELECT @level = CASE
		WHEN @status < 256 THEN @status & 0x07
		ELSE 0
	END

	--
	-- get current status
	--
	DECLARE @c_dt BIGINT,
			@c_seq INT,
			@c_pcdt BIGINT,
			@c_status SMALLINT,
	        @c_real_status SMALLINT,
			@c_level SMALLINT

	SELECT @c_dt = time,
		   @c_seq = swiftnum,
		   @c_pcdt = createtime,
		   @c_status = raw_status,
	       @c_real_status = real_status,
		   @c_level = alarm_level
	FROM mpoint_realtime_status
	WHERE mpoint_id = @mpid

	IF @c_status = @OFFLINE AND @status = @OFFLINE
	BEGIN
		RETURN
	END

	--
	-- get pre status
	--
	DECLARE @p_dt BIGINT,
		    @p_seq INT,
			@p_pcdt BIGINT,
	        @p_status SMALLINT,
			@p_real_status SMALLINT,
			@p_level SMALLINT

	SELECT @p_dt = time,
		   @p_seq = swiftnum,
	       @p_pcdt = createtime,
	       @p_status = raw_status,
	       @p_real_status = real_status,
		   @p_level = alarm_level
	FROM mpoint_pre_status
	WHERE mpoint_id = @mpid

	SELECT @dt = CASE
		WHEN @c_status = @ON THEN @c_dt + 1
		WHEN @status = @POWERON THEN @dt - 1
		ELSE @dt
	END

	--
	--
	--
	IF @status = @ONLINE
	BEGIN
		UPDATE mpoint_realtime_status
		SET time = @p_dt,
		    swiftnum = @p_seq,
			createtime = @p_pcdt,
		    raw_status = @p_status,
		    real_status = @p_real_status,
			alarm_level = @p_level
		WHERE mpoint_id = @mpid
		  AND real_status = @OFFLINE
	END
	ELSE
	BEGIN
		IF @dt > @c_dt OR (@dt = @c_dt AND @seq > @c_seq)
		BEGIN
			UPDATE mpoint_realtime_status
			SET time = @dt,
			    swiftnum = @seq,
			    createtime = @pcdt,
			    raw_status = @status,
				real_status = @real_status,
				alarm_level = @level
			WHERE mpoint_id = @mpid

			IF @status != @OFFLINE
			BEGIN
				UPDATE mpoint_pre_status
				SET time = @dt,
				    swiftnum = @seq,
				    createtime = @pcdt,
				    raw_status = @status,
					real_status = @real_status,
					alarm_level = @level
				WHERE mpoint_id = @mpid
			END
		END
	END
END



GO
/****** Object:  StoredProcedure [dbo].[new_channel_status]    Script Date: 2018/11/8 13:59:18 ******/
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
	@dt BIGINT,
	@seq BIGINT,
	@pcdt BIGINT,
	@status SMALLINT,
	@data float
AS
BEGIN
	SET NOCOUNT ON

	DECLARE @GND_H TINYINT = 8
	DECLARE @GND_L TINYINT = 12
	DECLARE @WS TINYINT = 9

	DECLARE @OFF SMALLINT = 257
	DECLARE @ON SMALLINT = 258
	DECLARE @OFFLINE SMALLINT = 256
	DECLARE @ONLINE SMALLINT = 259
	DECLARE @POWEROFF SMALLINT = 261
	DECLARE @POWERON SMALLINT = 260

    --
	--  check mpoint exists
	--
	DECLARE @mpid INT = NULL

	SELECT @mpid = mp.id
	FROM mpoint AS mp
	LEFT JOIN channels_info AS ci
	ON mp.ciid = ci.id
	LEFT JOIN devices_info AS di
	ON ci.device_id = di.id
	WHERE di.sn = @sn
	  AND ci.slot = @slot
	  AND ci.port = @port
	  AND ci.type = @type
	  AND mp.endtime = 0

	IF @mpid IS NULL
	BEGIN
		RETURN
	END

	--
	-- process status
	--
	DECLARE @real_status SMALLINT, @level SMALLINT

	SELECT @real_status = CASE
		WHEN @status < 256 THEN
		CASE
			WHEN @type = @WS THEN
			CASE
				WHEN @status & 0x80 = 0x80 THEN 0x80
				WHEN @status & 0x50 = 0x50 THEN 0x50
				WHEN @status & 0x40 = 0x40 THEN 0x40
				WHEN @status & 0x20 = 0x20 THEN 0x20
				ELSE 0x00
			END
			ELSE
			CASE
				WHEN @status & 0x20 = 0x20 THEN 0x20
				ELSE 0x00
			END
		END
		ELSE @status
	END

	SELECT @level = CASE
		WHEN @status < 256 THEN @status & 0x07
		ELSE 0
	END

	--
	-- get current status
	--
	DECLARE @c_dt BIGINT,
			@c_seq INT,
			@c_pcdt BIGINT,
			@c_status SMALLINT,
	        @c_real_status SMALLINT,
			@c_level SMALLINT,
			@c_data float

	SELECT @c_dt = time,
		   @c_seq = swiftnum,
		   @c_pcdt = createtime,
		   @c_status = raw_status,
	       @c_real_status = real_status,
		   @c_level = alarm_level,
		   @c_data = data
	FROM mpoint_realtime_status
	WHERE mpoint_id = @mpid

	IF @c_status = @OFFLINE AND @status = @OFFLINE
	BEGIN
		RETURN
	END

	--
	-- get pre status
	--
	DECLARE @p_dt BIGINT,
		    @p_seq INT,
			@p_pcdt BIGINT,
	        @p_status SMALLINT,
			@p_real_status SMALLINT,
			@p_level SMALLINT,
			@p_data float

	SELECT @p_dt = time,
		   @p_seq = swiftnum,
	       @p_pcdt = createtime,
	       @p_status = raw_status,
	       @p_real_status = real_status,
		   @p_level = alarm_level,
		   @p_data = data
	FROM mpoint_pre_status
	WHERE mpoint_id = @mpid

	SELECT @dt = CASE
		WHEN @c_status = @ON THEN @c_dt + 1
		WHEN @status = @POWERON THEN @dt - 1
		ELSE @dt
	END

	INSERT INTO mpoint_status (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime, data)
	VALUES (@mpid, @seq, @status, @real_status, @level, @dt, @pcdt, @data)

	IF @status > 255 AND @status != @OFFLINE AND @status != @ONLINE
	BEGIN
		RETURN
	END

	IF @status = @ONLINE
	BEGIN
		UPDATE mpoint_realtime_status
		SET time = @p_dt,
		    swiftnum = @p_seq,
			createtime = @p_pcdt,
		    raw_status = @p_status,
		    real_status = @p_real_status,
			alarm_level = @p_level,
			data = @p_data
		WHERE mpoint_id = @mpid
		  AND real_status = @OFFLINE
	END
	ELSE
	BEGIN
		IF @dt > @c_dt OR (@dt = @c_dt AND @seq > @c_seq)
		BEGIN
			UPDATE mpoint_realtime_status
			SET time = @dt,
			    swiftnum = @seq,
			    createtime = @pcdt,
			    raw_status = @status,
				real_status = @real_status,
				alarm_level = @level,
				data = @data
			WHERE mpoint_id = @mpid

			IF @status != @OFFLINE
			BEGIN
				UPDATE mpoint_pre_status
				SET time = @dt,
				    swiftnum = @seq,
				    createtime = @pcdt,
				    raw_status = @status,
					real_status = @real_status,
					alarm_level = @level,
					data = @data
				WHERE mpoint_id = @mpid
			END
		END
	END

END



GO
/****** Object:  StoredProcedure [dbo].[new_device_realtime_status]    Script Date: 2018/11/8 13:59:18 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO

-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-08-11
-- Description:	new_device_realtime_status
-- =============================================
CREATE PROCEDURE [dbo].[new_device_realtime_status]
	@sn NVARCHAR(20),
	@dt BIGINT,
	@status SMALLINT
AS
BEGIN
	SET NOCOUNT ON
	
	DECLARE @slot TINYINT, @port TINYINT, @type TINYINT

	DECLARE cc CURSOR FOR
	SELECT ci.slot, ci.port, ci.type
	FROM mpoint AS mp
	LEFT JOIN channels_info AS ci
	ON mp.ciid = ci.id
	LEFT JOIN devices_info AS di
	ON ci.device_id = di.id
	WHERE di.sn = @sn
	  AND mp.endtime = 0

	OPEN cc

	FETCH NEXT FROM cc
	INTO @slot, @port, @type

	WHILE @@FETCH_STATUS = 0
	BEGIN
		EXEC new_channel_realtime_status @sn, @slot, @port, @type, @dt, 0, @dt, @status

		FETCH NEXT FROM cc
		INTO @slot, @port, @type
	END

	CLOSE cc

	DEALLOCATE cc
END



GO
/****** Object:  StoredProcedure [dbo].[new_device_status]    Script Date: 2018/11/8 13:59:18 ******/
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
	SET NOCOUNT ON
	
	DECLARE @slot TINYINT, @port TINYINT, @type TINYINT

	DECLARE cc CURSOR FOR
	SELECT ci.slot, ci.port, ci.type
	FROM mpoint AS mp
	LEFT JOIN channels_info AS ci
	ON mp.ciid = ci.id
	LEFT JOIN devices_info AS di
	ON ci.device_id = di.id
	WHERE di.sn = @sn
	  AND mp.endtime = 0

	OPEN cc

	FETCH NEXT FROM cc
	INTO @slot, @port, @type

	WHILE @@FETCH_STATUS = 0
	BEGIN
		EXEC new_channel_status @sn, @slot, @port, @type, @dt, 0, @dt, @status, NULL

		FETCH NEXT FROM cc
		INTO @slot, @port, @type
	END

	CLOSE cc

	DEALLOCATE cc
END






GO
/****** Object:  StoredProcedure [dbo].[new_mpoint]    Script Date: 2018/11/8 13:59:18 ******/
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
	ELSE
	BEGIN
		RETURN
	END

	INSERT INTO mpoint_realtime_status (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
	VALUES (@mpid, 0, 0, 0, 0, @dt + 1, @dt + 1)

	INSERT INTO mpoint_pre_status (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
	VALUES (@mpid, 0, 0, 0, 0, @dt + 1, @dt + 1)

	INSERT INTO mpoint_status (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
	VALUES (@mpid, 0, 258, 258, 0, @dt, @dt), (@mpid, 0, 0, 0, 0, @dt + 1, @dt + 1)
END






GO
/****** Object:  StoredProcedure [dbo].[offlineDevices]    Script Date: 2018/11/8 13:59:18 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO

-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-10-27
-- Description:	offline devices
-- =============================================
CREATE PROCEDURE [dbo].[offlineDevices]
	@dt BIGINT
AS
BEGIN
	SET NOCOUNT ON;

	DECLARE @OFFLINE SMALLINT = 256

	DECLARE @sn NVARCHAR(20)

	DECLARE od_c CURSOR FOR
	SELECT sn FROM devices_info

	OPEN od_c

	FETCH NEXT FROM od_c
	INTO @sn

	WHILE @@FETCH_STATUS = 0
	BEGIN
		EXEC new_device_status @sn, @dt, @OFFLINE

		FETCH NEXT FROM od_c
		INTO @sn
	END

	CLOSE od_c

	DEALLOCATE od_c
END


GO
/****** Object:  StoredProcedure [dbo].[openMpoint]    Script Date: 2018/11/8 13:59:18 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO

-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-11-07
-- Description:	openMpoint
-- =============================================
CREATE PROCEDURE [dbo].[openMpoint]
	@sn NVARCHAR(20),
	@slot TINYINT,
	@port TINYINT,
	@type TINYINT,
	@pid INT,
	@name NVARCHAR(20),
	@dt BIGINT
AS
BEGIN
	SET NOCOUNT ON;

	DECLARE @ON INT = 258

	DECLARE @did INT = NULL

	SELECT @did = id
	FROM devices_info
	WHERE sn = @sn

	IF @did IS NULL
	BEGIN
		RETURN 0
	END

	DECLARE @cid INT = NULL

	SELECT @cid = id
	FROM channels_info
	WHERE device_id = @did AND
	      slot = @slot AND
		  port = @port AND
		  type = @type

	IF @cid IS NULL
	BEGIN
		INSERT INTO channels_info (device_id, slot, port, type, createtime, updatetime)
		VALUES (@did, @slot, @port, @type, @dt, @dt)

		SELECT @cid = SCOPE_IDENTITY()
	END

	DECLARE @mid INT = NULL

	SELECT @mid = id
	FROM mpoint
	WHERE ciid = @cid AND
	      pid = @pid AND
		  endtime = 0

	IF @mid IS NULL
	BEGIN
		INSERT INTO mpoint (pid, ciid, name, starttime, endtime)
		VALUES (@pid, @cid, @name, @dt, 0)

		SELECT @mid = SCOPE_IDENTITY()
	END
	ELSE
	BEGIN
		RETURN @mid
	END

	INSERT INTO mpoint_realtime_status (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
	VALUES (@mid, 0, 0, 0, 0, @dt + 1, @dt + 1)

	INSERT INTO mpoint_pre_status (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
	VALUES (@mid, 0, 0, 0, 0, @dt + 1, @dt + 1)

	INSERT INTO mpoint_status (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime)
	VALUES (@mid, 0, @ON, @ON, 0, @dt, @dt), (@mid, 0, 0, 0, 0, @dt + 1, @dt + 1)

	RETURN @mid
END






GO
/****** Object:  StoredProcedure [dbo].[updateDevice]    Script Date: 2018/11/8 13:59:18 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-10-31
-- Description:	updateDevice
-- =============================================
CREATE PROCEDURE [dbo].[updateDevice]
	@sn VARCHAR(20),
	@psn VARCHAR(20), -- ''
	@ip VARCHAR(20), -- ''
	@port INT, -- 0
	@devaddr VARCHAR(20), -- ''
	@dt BIGINT
AS
BEGIN
	SET NOCOUNT ON;

	DECLARE @pid INT = NULL
	IF @psn = ''
	BEGIN
		SET @pid = 0
	END
	ELSE
	BEGIN
		SELECT @pid = id FROM devices_info WHERE sn = @psn
	END
	
	IF @pid IS NULL
	BEGIN
		RETURN
	END

	DECLARE @type VARCHAR(20)
	DECLARE @level INT = 1

	IF @devaddr != ''
	BEGIN
		SET @type = 'LORA'
	END
	ELSE
	BEGIN
		SET @type = 'KEDAS'
	END

	IF @psn != ''
	BEGIN
		SET @type = 'ZIGBEE'
		SET @level = 2
	END

	DECLARE @id INT = NULL
	SELECT @id = id FROM devices_info WHERE sn = @sn
	IF @id IS NOT NULL
	BEGIN
		UPDATE devices_info
		SET parent_id = @pid,
			ip = @ip,
			port = @port,
		    devaddr = @devaddr,
			updatetime = @dt
		WHERE id = @id

		UPDATE devices_info
		SET ip = @ip,
			port = @port,
		    devaddr = @devaddr,
			updatetime = @dt
		WHERE parent_id = @id
	END
	ELSE
	BEGIN
		INSERT INTO devices_info
		(parent_id, sn, ip, port, type, status, level, createtime, updatetime, devaddr)
		VALUES
		(@pid, @sn, @ip, @port, @type, 0, @level, @dt, @dt, @devaddr)

		SET @id = SCOPE_IDENTITY()

		INSERT INTO device_rssi
		(device_id, rssi, updatetime)
		VALUES
		(@id, 1, @dt)
	END

	IF @type != 'ZIGBEE'
	BEGIN
		UPDATE devices_info
		SET ip = '',
			port = 0
		WHERE id != @id AND parent_id != @id AND ip = @ip AND port = @port

		UPDATE devices_info
		SET devaddr = ''
		WHERE id != @id AND parent_id != @id AND devaddr = @devaddr
	END
END

GO
/****** Object:  StoredProcedure [dbo].[updateDeviceIpPort]    Script Date: 2018/11/8 13:59:18 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO

-- =============================================
-- Author:		Zhiwen He
-- Create date: 2018-08-08
-- Description:	updateDeviceIpPort
-- =============================================
CREATE PROCEDURE [dbo].[updateDeviceIpPort]
	@sn NVARCHAR(20),
	@ip CHAR(15),
	@port INT
AS
BEGIN
	-- SET NOCOUNT ON added to prevent extra result sets from
	-- interfering with SELECT statements.
	SET NOCOUNT ON;

    UPDATE devices_info
	SET ip = @ip
	WHERE sn = @sn
END



GO
/****** Object:  StoredProcedure [dbo].[updateOfflineTime]    Script Date: 2018/11/8 13:59:18 ******/
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
/****** Object:  UserDefinedFunction [dbo].[fn_ConvertTime]    Script Date: 2018/11/8 13:59:18 ******/
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
/****** Object:  UserDefinedFunction [dbo].[fn_GetPlace]    Script Date: 2018/11/8 13:59:18 ******/
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
/****** Object:  Table [dbo].[alarm_delay_config]    Script Date: 2018/11/8 13:59:18 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[alarm_delay_config](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[type] [int] NOT NULL,
	[delay_time] [int] NOT NULL,
 CONSTRAINT [PK_alarm_delay_config] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[channels_info]    Script Date: 2018/11/8 13:59:19 ******/
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
/****** Object:  Table [dbo].[device_param]    Script Date: 2018/11/8 13:59:19 ******/
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
/****** Object:  Table [dbo].[device_rssi]    Script Date: 2018/11/8 13:59:19 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[device_rssi](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[device_id] [int] NOT NULL,
	[rssi] [smallint] NOT NULL,
	[updatetime] [bigint] NOT NULL
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[devices_info]    Script Date: 2018/11/8 13:59:19 ******/
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
	[devaddr] [varchar](20) NULL,
	[port] [int] NULL,
 CONSTRAINT [PK_devices_info] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
SET ANSI_PADDING OFF
GO
/****** Object:  Table [dbo].[mpoint]    Script Date: 2018/11/8 13:59:19 ******/
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
/****** Object:  Table [dbo].[mpoint_data]    Script Date: 2018/11/8 13:59:19 ******/
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
/****** Object:  Table [dbo].[mpoint_pre_status]    Script Date: 2018/11/8 13:59:19 ******/
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
	[data] [float] NULL,
 CONSTRAINT [PK_mpoint_pre_status] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[mpoint_realtime_status]    Script Date: 2018/11/8 13:59:19 ******/
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
	[data] [float] NULL,
 CONSTRAINT [PK_mpoint_realtime_status] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[mpoint_status]    Script Date: 2018/11/8 13:59:19 ******/
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
	[data] [float] NULL,
 CONSTRAINT [PK_mpoint_status] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
) ON [PRIMARY]

GO
/****** Object:  Table [dbo].[place]    Script Date: 2018/11/8 13:59:19 ******/
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
/****** Object:  Table [dbo].[push_list]    Script Date: 2018/11/8 13:59:19 ******/
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
/****** Object:  Table [dbo].[user_default_report_setting]    Script Date: 2018/11/8 13:59:19 ******/
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
/****** Object:  Table [dbo].[webservice_user]    Script Date: 2018/11/8 13:59:19 ******/
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
/****** Object:  Table [dbo].[ws_worktime_daily]    Script Date: 2018/11/8 13:59:19 ******/
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
/****** Object:  View [dbo].[chan_status_without_online]    Script Date: 2018/11/8 13:59:19 ******/
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
	sn, slot, port, type,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, mpoint_id, ciid, pid, time, status, data
FROM (
	SELECT
		p0.name AS plant,
		p1.name AS workshop,
		p2.name AS region,
		p3.name AS line,
		p4.name AS station,
		mp.name AS mpoint,
		di.sn, ci.slot, ci.port, ci.type,
		p0.id AS plantid,
		p1.id AS workshopid,
		p2.id AS regionid,
		p3.id AS lineid,
		p4.id AS stationid,
		ms.id, ms.mpoint_id, mp.ciid, mp.pid, time,
		LAG(real_status) OVER (PARTITION BY ms.mpoint_id ORDER BY time) AS pstatus,
		real_status as status,
		LEAD(real_status) OVER (PARTITION BY ms.mpoint_id ORDER BY time) AS nstatus,
		data
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
/****** Object:  View [dbo].[chan_status_without_online_no_dup]    Script Date: 2018/11/8 13:59:19 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO







CREATE VIEW [dbo].[chan_status_without_online_no_dup] AS
SELECT
plant,
	workshop,
	region,
	line,
	station,
	mpoint,
	sn, slot, port, type,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, mpoint_id, ciid, pid, time, status, data
FROM (
	SELECT
	plant,
	workshop,
	region,
	line,
	station,
	mpoint,
	sn, slot, port, type,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, mpoint_id, ciid, pid, time,
	LAG(status) OVER (PARTITION BY mpoint_id ORDER BY time) AS pstatus,
	status,
	LEAD(status) OVER (PARTITION BY mpoint_id ORDER BY time) AS nstatus,
	data
	FROM chan_status_without_online
) AS t
WHERE pstatus IS NULL OR pstatus != status












GO
/****** Object:  View [dbo].[chan_status]    Script Date: 2018/11/8 13:59:19 ******/
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
	sn, slot, port, type AS porttype,
	plantid,
	workshopid,
	regionid,
	lineid,
	stationid,
	id, ciid, pid,
	time AS starttime,
	LEAD(time) OVER (PARTITION BY mpoint_id ORDER BY time) AS endtime,
	status,
	data
FROM chan_status_without_online_no_dup











GO
/****** Object:  View [dbo].[view_mpoint_status_without_online]    Script Date: 2018/11/8 13:59:19 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO







CREATE view [dbo].[view_mpoint_status_without_online]
as
select id, mpoint_id, real_status, alarm_level, time
 from
(SELECT id, mpoint_id, real_status, alarm_level, time,
LEAD(real_status) OVER (PARTITION BY mpoint_id ORDER BY time) AS nstatus

  FROM mpoint_status 

)t
  where
  real_status != 259 AND (real_status != 256 OR (nstatus IS NULL OR nstatus = 260)) AND real_status != 260











GO
/****** Object:  View [dbo].[view_mpoint_status_without_online_no_dup]    Script Date: 2018/11/8 13:59:19 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO








CREATE VIEW [dbo].[view_mpoint_status_without_online_no_dup] AS

SELECT id, mpoint_id,  real_status,time as starttime, LEAD(time,1,0) OVER (PARTITION BY mpoint_id ORDER BY time) AS endtime 
FROM (
	SELECT
		id, mpoint_id,  t.time, real_status
	FROM (
		SELECT
		id, mpoint_id, time,
		LAG(real_status) OVER (PARTITION BY mpoint_id ORDER BY time) AS pstatus,
		real_status,
		LEAD(real_status) OVER (PARTITION BY mpoint_id ORDER BY time) AS nstatus
		FROM view_mpoint_status_without_online
	) AS t
	WHERE pstatus IS NULL OR pstatus != real_status )t1













GO
/****** Object:  View [dbo].[view_mpoint_status]    Script Date: 2018/11/8 13:59:19 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO



CREATE view [dbo].[view_mpoint_status]
as
select id, mpoint_id, real_status, alarm_level, time as starttime,
lead(time,1,0) over(partition by mpoint_id order by time) as endtime
 from
(SELECT id, mpoint_id, real_status, alarm_level, time,
LEAD(real_status) OVER (PARTITION BY mpoint_id ORDER BY time) AS nstatus

  FROM mpoint_status 

)t
  where
  real_status != 259 AND (real_status != 256 OR (nstatus IS NULL OR nstatus = 260)) AND real_status != 260







GO
/****** Object:  Index [IX_mpoint_status]    Script Date: 2018/11/8 13:59:19 ******/
CREATE NONCLUSTERED INDEX [IX_mpoint_status] ON [dbo].[mpoint_status]
(
	[mpoint_id] ASC,
	[time] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, SORT_IN_TEMPDB = OFF, DROP_EXISTING = OFF, ONLINE = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON) ON [PRIMARY]
GO
USE [master]
GO
ALTER DATABASE [iotdb] SET  READ_WRITE 
GO
