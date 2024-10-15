package main

import (
	"fmt"
	"os"
)

func main() {
	username := os.Getenv("DATABASE_USERNAME")
	password := os.Getenv("DATABASE_PASSWORD")
	host := os.Getenv("DATABASE_HOST")
	database := os.Getenv("DATABASE_NAME")
	port := os.Getenv("DATABASE_PORT")

	dsn := fmt.Sprintf("host=%s user=%s password=%s dbname=%s port=%s sslmode=disable", host, username, password, database, port)
	err := os.Remove("./config.toml")
	if err != nil {
		fmt.Println("config.toml does not exist")
	}
	data, err := os.ReadFile("./sample_config.toml")
	if err != nil {
		fmt.Println("sample_config.toml does not exist")
	}
	config := string(data)
	config += "\n[datastore.postgres]\ndsn = \"" + dsn + "\"\n"
	err = os.WriteFile("./config.toml", []byte(config), 0644)
	if err != nil {
		fmt.Println("config.toml cannot be written")
	}
}
