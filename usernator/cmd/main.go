package main

import (
	"usernator/internal/grpc"
	"usernator/internal/server"
)

func main() {

	app := server.CreateServer("./config.json")

	go grpc.StartGrpcServer()
	err := app.Listen(":3000")
	if err != nil {
		panic(err.Error())
	}
}
