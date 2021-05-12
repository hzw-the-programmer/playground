package main

import (
	"bufio"
	"database/sql"
	"flag"
	"fmt"
	"log"
	"os"

	_ "github.com/denisenkom/go-mssqldb"
)

func main() {
	server := flag.String("server", "", "database server")
	user := flag.String("user", "", "database user")
	password := flag.String("password", "", "database password")
	port := flag.Int("port", 1433, "database port")
	database := flag.String("database", "", "database name")
	config := flag.String("config", "", "config file")

	flag.Parse()

	if *server == "" || *user == "" ||
		*password == "" || *database == "" ||
		*config == "" {
		logAndWaitThenExit("server, user, password, database, config must be specified")
	}

	constr := fmt.Sprintf(
		"server=%s;user id=%s;password=%s;port=%d;database=%s",
		*server, *user, *password, *port, *database,
	)

	db, err := sql.Open("mssql", constr)
	if err != nil {
		logAndWaitThenExit(err)
	}
	defer db.Close()

	err = db.Ping()
	if err != nil {
		logAndWaitThenExit(err)
	}

	_, err = db.Exec("TRUNCATE TABLE isolated_dev")
	if err != nil {
		logAndWaitThenExit(err)
	}

	f, err := os.Open(*config)
	if err != nil {
		logAndWaitThenExit(err)
	}
	defer f.Close()

	sns := make(map[string]bool)
	s := bufio.NewScanner(f)
	for s.Scan() {
		sn := s.Text()
		_, ok := sns[sn]
		if ok {
			continue
		}
		_, err = db.Exec("INSERT INTO isolated_dev VALUES (?)", sn)
		if err != nil {
			logAndWaitThenExit(err)
		}
		sns[sn] = true
	}

	logAndWaitThenExit("Success!!!")
}

func logAndWaitThenExit(msg interface{}) {
	log.Print(msg)
	stdin := bufio.NewReader(os.Stdin)
	stdin.ReadLine()
	os.Exit(1)
}
