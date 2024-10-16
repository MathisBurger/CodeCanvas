package main

import (
	"usernator/internal/server"
)

func main() {

	app := server.CreateServer("./config.json")
	err := app.Listen(":3000")
	if err != nil {
		panic(err.Error())
	}
}
