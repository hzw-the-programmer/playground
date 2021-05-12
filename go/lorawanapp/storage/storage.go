/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package storage

import (
	"database/sql"
	"log"
	"time"

	"lorawanapp/config"
)

var db *sql.DB

func SetUpStorage() {
	var err error
	db, err = sql.Open("mysql", config.C.DSN)
	if err != nil {
		panic(err)
	}

	for {
		if err = db.Ping(); err != nil {
			log.Print(err)
			time.Sleep(2 * time.Second)
		} else {
			break
		}
	}
}

func GetDB() *sql.DB {
	return db
}

func GetMPID(sn string, slot, port, typ uint8) (uint64, error) {
	rows, err := db.Query(`
		SELECT mp.id FROM mpoint AS mp
		LEFT JOIN channels_info AS ci ON mp.ciid = ci.id
		LEFT JOIN devices_info AS di ON ci.device_id = di.id
		WHERE di.sn = ? AND ci.slot = ? AND ci.port = ? AND ci.type = ? AND mp.endtime = 0
	`, sn, slot, port, typ)

	if err != nil {
		return 0, err
	}

	var mpid uint64

	for rows.Next() == true {
		err = rows.Scan(&mpid)
		if err != nil {
			return 0, err
		}
	}

	rows.Close()

	return mpid, nil
}

func UpdateRealTimeStatus(mpid, seq uint64, status int, t int64, data float32) (int64, error) {
	rs := status
	al := 0

	r, err := db.Exec(`
		UPDATE mpoint_realtime_status
		SET swiftnum = ?,
			raw_status = ?,
			real_status = ?,
			alarm_level = ?,
			time = ?,
			data = ?
		WHERE mpoint_id = ?
	`, seq, status, rs, al, t, data, mpid)

	if err != nil {
		return 0, err
	}

	n, err := r.RowsAffected()

	if err != nil {
		return 0, err
	}

	return n, nil
}

func InsertStatus(mpid, seq uint64, status int, t int64, data float32) (int64, error) {
	rs := status
	al := 0

	r, err := db.Exec(`
		INSERT INTO mpoint_status (
			mpoint_id,
			swiftnum,
			raw_status,
			real_status,
			alarm_level,
			time,
			createtime,
			data
		) VALUES (
			?, ?, ?, ?, ?, ?, ?, ?
		)
	`, mpid, seq, status, rs, al, t, t, data)

	if err != nil {
		return 0, err
	}

	n, err := r.LastInsertId()

	if err != nil {
		return 0, err
	}

	return n, nil
}

func InsertData(mpid, seq uint64, t int64, data float32) (int64, error) {
	r, err := db.Exec(`
		INSERT INTO mpoint_data (
			mpoint_id,
			swiftnum,
			data,
			time,
			createtime,
			is_pad
		) VALUES (
			?, ?, ?, ?, ?, ?
		)
	`, mpid, seq, data, t, t, 0)

	if err != nil {
		return 0, err
	}

	n, err := r.LastInsertId()

	if err != nil {
		return 0, err
	}

	return n, nil
}
