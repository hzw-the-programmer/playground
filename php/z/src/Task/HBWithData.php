<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Task;

use Hzw\Task;
use Hzw\Channel;
use Hzw\Config;

class HBWithData extends Task {
    static $cntx;
    static $sqls = [
        "
            EXEC heartBeatWithData :sn, :slot, :port, :type, :dt, :pcdt, :seq
            , :avgData, :status, :realStatus, :level, :data
        ",
        "
            UPDATE ds
            SET ds.rssi = :signal,
                ds.updatetime = :dt
            FROM device_rssi AS ds
            LEFT JOIN devices_info AS di
            ON ds.device_id = di.id
            WHERE di.sn = :sn
        "
    ];
    static $stmts = [];
    static $className;

    public $sn;
    public $seq;
    public $dt;
    public $pcdt;
    public $channels;

    public $signal;

    function __construct($sn, $seq, $dt, $pcdt, $channels, $signal) {
        $this->sn = $sn;
        $this->seq = $seq;
        $this->dt = $dt;
        $this->pcdt = $pcdt;
        $this->channels = $channels;

        $this->signal = $signal;
    }

    function onTransaction($dbh) {
        $logger = static::$cntx->getLogger();

        $stmt = static::$stmts[0];

        $sn = $this->sn;
        $seq = $this->seq;
        $dt = $this->dt;
        $pcdt = $this->pcdt;
        $channels = $this->channels;

        $stmt->bindParam(':sn', $sn);
        $stmt->bindParam(":seq", $seq);
        $stmt->bindParam(":dt", $dt);
        $stmt->bindParam(":pcdt", $pcdt);

        foreach ($channels as $channel) {
            $slot = $channel->getSlot();
            $port = $channel->getPort();
            $type = $channel->getType();
            $avgData = $channel->getAvgData();
            $status = $channel->getStatus();
            $realStatus = $channel->getRealStatus();
            $level = $channel->getLevel();
            $data = $channel->getData();

            $stmt->bindParam(':slot', $slot);
            $stmt->bindParam(':port', $port);
            $stmt->bindParam(':type', $type);
            $stmt->bindParam(':avgData', $avgData);
            $stmt->bindParam(":status", $status);
            $stmt->bindParam(":realStatus", $realStatus);
            $stmt->bindParam(":level", $level);
            $stmt->bindParam(":data", $data);

            $stmt->execute();
        }

        $stmt = static::$stmts[1];

        $stmt->bindParam(':signal', $this->signal);
        $stmt->bindParam(":dt", $dt);
        $stmt->bindParam(":sn", $sn);

        $stmt->execute();
    }
}

/*
CREATE PROCEDURE [dbo].[heartBeatWithData]
	@sn VARCHAR(20),
	@slot TINYINT,
	@port TINYINT,
	@type TINYINT,
	@dt BIGINT,
	@pcdt BIGINT,
	@seq INT,
	@avgData FLOAT,
	@status TINYINT,
	@realStatus TINYINT,
	@level TINYINT,
	@data FLOAT
AS
BEGIN
	-- SET NOCOUNT ON added to prevent extra result sets from
	-- interfering with SELECT statements.
	SET NOCOUNT ON;

    DECLARE @mpid INT = NULL

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

	IF @avgData IS NOT NULL --@seq % 10 = 0
	BEGIN
		INSERT INTO mpoint_data (mpoint_id, swiftnum, data, time, createtime)
		VALUES (@mpid, @seq, @avgData, @dt, @pcdt)
	END

	DECLARE @c_status SMALLINT

	SELECT @c_status = raw_status
	FROM mpoint_realtime_status
	WHERE mpoint_id = @mpid

	IF @c_status != @status
	BEGIN
		UPDATE mpoint_realtime_status
		SET time = @dt,
			swiftnum = @seq,
			raw_status = @status,
			real_status = @realStatus,
			alarm_level = @level,
			data = @data,
			createtime = @pcdt
		WHERE mpoint_id = @mpid

		INSERT INTO mpoint_status (mpoint_id, swiftnum, raw_status, real_status, alarm_level, time, createtime, data)
		VALUES (@mpid, @seq, @status, @realStatus, @level, @dt, @pcdt, @data)
	END
END 
*/
