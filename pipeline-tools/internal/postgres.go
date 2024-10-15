package internal

import "database/sql"

func InitPostgres() {
	db, err := sql.Open("postgres", "postgres://admin:admin@postgres:5432/postgres?sslmode=disable")
	if err != nil {
		panic(err)
	}
	defer db.Close()
	db.Exec("CREATE DATABASE usernator")
	db.Exec("CREATE DATABASE tasky")
	db.Exec("CREATE DATABASE executor")
}
