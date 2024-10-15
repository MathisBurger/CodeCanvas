package internal

import (
	"database/sql"
	_ "github.com/lib/pq"
)

func InitPostgres() {
	db, err := sql.Open("postgres", "postgres://admin:admin@127.0.0.1:5432/postgres?sslmode=disable")
	if err != nil {
		panic(err)
	}
	defer db.Close()
	db.Exec("CREATE DATABASE usernator")
	db.Exec("CREATE DATABASE tasky")
	db.Exec("CREATE DATABASE executor")
}
