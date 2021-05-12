package main

import (
	"database/sql"
	_ "github.com/go-sql-driver/mysql"
	"fmt"
)

//go get -u github.com/go-sql-driver/mysql
//go env
//GOPATH="/home/zhiwenhe/go"
//ls ~/go/pkg/linux_amd64/github.com/go-sql-driver/

func main() {
	db, err := sql.Open("mysql", "hzw:ASDf@123@/website_skeleton")
	if err != nil {
		panic(err.Error())
	}
	defer db.Close()

	err = db.Ping()
	if err != nil {
		panic(err.Error())
	}

	stmt, err := db.Prepare("SELECT sn, devaddr FROM devices_info")
	if err != nil {
		panic(err.Error())
	}
	defer stmt.Close()

	rows, err := stmt.Query()
	if err != nil {
		panic(err.Error())
	}

	columns, err := rows.Columns()
	if err != nil {
		panic(err.Error())
	}

	values := make([]sql.RawBytes, len(columns))

	scanArgs := make([]interface{}, len(values))
	for i := range values {
		scanArgs[i] = &values[i]
	}

	for rows.Next() {
		err = rows.Scan(scanArgs...)
		if err != nil {
			panic(err.Error())
		}

		var value string
		for i, col := range values {
			if col == nil {
				value = "NULL"
			} else {
				value = string(col)
			}
			fmt.Println(columns[i], ": ", value)
		}
		fmt.Println("-------------------")
	}
	if err = rows.Err(); err != nil {
		panic(err.Error())
	}
}
