package main

import (
	"database/sql"
	"log"

	_ "github.com/go-sql-driver/mysql"
)

func main() {
	db, err := sql.Open("mysql", "root:iDAS@123@tcp(10.0.37.57)/kaifaiot?parseTime=true")
	if err != nil {
		panic(err)
	}
	defer db.Close()

	err = db.Ping()
	if err != nil {
		panic(err)
	}

	rows, err := db.Query(`
		SELECT sn, mp.id, slot, ci.port, ci.type FROM mpoint AS mp
		LEFT JOIN channels_info AS ci ON mp.ciid = ci.id
		LEFT JOIN devices_info AS di ON ci.device_id = di.id
		WHERE sn = "02112191300107" AND mp.endtime = 0

	`)
	if err != nil {
		panic(err)
	}
	defer rows.Close()
	
	/*
	columns, err := rows.Columns()
	if err != nil {
		panic(err)
	}
	log.Print(columns)
	*/

	var sn string
	var id, slot, port, typ int
	for rows.Next() == true {
		err = rows.Scan(&sn, &id, &slot, &port, &typ)
		if err != nil {
			panic(err)
		}
		log.Printf("%s %d %d %d %d", sn, id, slot, port, typ)
	}
}
