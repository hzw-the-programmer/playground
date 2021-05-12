package main

import (
	"database/sql"
	"fmt"
	"os"

	_ "github.com/lib/pq"
	"github.com/rubenv/sql-migrate"
)

const (
	dbhost = "DBHOST"
	dbport = "DBPORT"
	dbuser = "DBUSER"
	dbpass = "DBPASS"
	dbname = "DBNAME"
)

var db *sql.DB

// DBHOST="127.0.0.1" DBPORT=5432 DBUSER=demo_role DBPASS=asdf DBNAME=postgres ./build/main
func main() {
	initDb()
	defer db.Close()

	migrations := &migrate.MemoryMigrationSource{
		Migrations: []*migrate.Migration{
			&migrate.Migration{
				Id:   "123",
				Up:   []string{"CREATE TABLE zwh1 (id serial)"},
				Down: []string{"DROP TABLE zwh1"},
			},
		},
	}

	n, err := migrate.Exec(db, "postgres", migrations, migrate.Up)
	if err != nil {
		panic(err)
	}
	fmt.Printf("Applied %d Migrations!\n", n)
}

func initDb() {
	config := dbConfig()
	var err error
	psqlInfo := fmt.Sprintf("host=%s port=%s user=%s"+
		" password=%s dbname=%s sslmode=disable",
		config[dbhost], config[dbport], config[dbuser],
		config[dbpass], config[dbname])
	db, err = sql.Open("postgres", psqlInfo)
	if err != nil {
		panic(err)
	}
	err = db.Ping()
	if err != nil {
		panic(err)
	}
	fmt.Println("Successfully connected!")
}

func dbConfig() map[string]string {
	config := make(map[string]string)

	host, ok := os.LookupEnv(dbhost)
	if !ok {
		panic(dbhost + " environment variable required but not set")
	}
	port, ok := os.LookupEnv(dbport)
	if !ok {
		panic(dbport + " environment variable required but not set")
	}
	user, ok := os.LookupEnv(dbuser)
	if !ok {
		panic(dbuser + " environment variable required but not set")
	}
	pass, ok := os.LookupEnv(dbpass)
	if !ok {
		panic(dbpass + " environment variable required but not set")
	}
	name, ok := os.LookupEnv(dbname)
	if !ok {
		panic(dbname + " environment variable required but not set")
	}

	config[dbhost] = host
	config[dbport] = port
	config[dbuser] = user
	config[dbpass] = pass
	config[dbname] = name

	return config
}
