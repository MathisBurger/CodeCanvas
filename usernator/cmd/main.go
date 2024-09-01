package main

import (
	"github.com/gofiber/fiber/v2"
	"usernator/internal/config"
	"usernator/internal/shared"
	"usernator/internal/startup"
)

func main() {
	conf, err := config.LoadConfiguration()
	if err != nil {
		panic(err.Error())
	}
	shared.Config = conf
	startup.Database()

	app := fiber.New(fiber.Config{})

	// TODO: Implement endpoints for basic user needs

	err = app.Listen(":3000")
	if err != nil {
		panic(err.Error())
	}
}
