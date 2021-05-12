<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw\Task;

use Hzw\Task;

class UpdateDevice extends Task {
	static $cntx;
	static $className;
    static $sqls = [
        'EXEC updateDevice :sn, :psn, :ip, :port, :devaddr, :dt'
    ];
    static $stmts = [];

    public $sn;
    public $psn;
    public $address;
    public $dt;

    function __construct($sn, $psn, $address, $dt) {
        $this->sn = $sn;
        $this->psn = $psn;
        $this->address = $address;
        $this->dt = $dt;
    }

    function onTransaction($dbh) {
        $stmt = static::$stmts[0];

        $sn = $this->sn;
        $psn = $this->psn;
        if (array_key_exists('devaddr', $this->address)) {
            $ip = '';
            $port = 0;
            $devaddr = $this->address['devaddr'];
        } else {
            $ip = $this->address['ip'];
            $port = $this->address['port'];
            $devaddr = '';
        }
        $dt = $this->dt;

        $stmt->bindParam(":sn", $sn);
        $stmt->bindParam(":psn", $psn);
        $stmt->bindParam(":ip", $ip);
        $stmt->bindParam(":port", $port);
        $stmt->bindParam(":devaddr", $devaddr);
        $stmt->bindParam(":dt", $dt);

        $stmt->execute();
    }
}

/*
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
*/
