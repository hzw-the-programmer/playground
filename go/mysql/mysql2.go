package main

import (
	"database/sql"
	"log"
	"time"

	_ "github.com/go-sql-driver/mysql"
	"github.com/rubenv/sql-migrate"
)

func main() {
	db, err := sql.Open("mysql", "root:asdF@123@/iot?parseTime=true")
	if err != nil {
		panic(err)
	}
	defer db.Close()

	err = db.Ping()
	if err != nil {
		panic(err)
	}

	migrations := &migrate.FileMigrationSource{
		Dir: "db/migrations",
	}
	_, err = migrate.Exec(db, "mysql", migrations, migrate.Up)
	if err != nil {
		panic(err)
	}

	sec := time.Now().Unix()

	/*
		insert, err := db.Prepare(`
			INSERT INTO devices_info
			(parent_id, sn, mac, ip, port, power, type, status, level, createtime, updatetime, devaddr, version)
			VALUES
			(0, ?, '', '', 0, 0, 1, 0, 1, ?, ?, '', '1.1.1')`)
		if err != nil {
			panic(err)
		}
		defer insert.Close()

		res, err := insert.Exec("20180123000001", sec, sec)
		if err != nil {
			panic(err)
		}
	*/

	res, err := db.Exec(`
		INSERT INTO devices_info
		(parent_id, sn, mac, ip, port, power, type, status, level, createtime, updatetime, devaddr, version)
		VALUES
		(0, ?, '', '', 0, 0, 1, 0, 1, ?, ?, '', '1.1.1')
	`, "20180123000002", sec, sec)
	if err != nil {
		panic(err)
	}

	id, err := res.LastInsertId()
	if err != nil {
		panic(err)
	}

	log.Print(id)
}
