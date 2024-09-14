package main

import (
	"github.com/gofiber/fiber/v2"
	"usernator/internal/config"
	"usernator/internal/controllers"
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
	startup.InitRabbitMQ()

	app := fiber.New(fiber.Config{})

	app.Post("/register", controllers.CreateUser)
	app.Get("/user/:id", controllers.GetUser)
	app.Post("/login", controllers.LoginUser)
	app.Post("/reset_password", controllers.ResetPassword)
	app.Post("/submit_password", controllers.SubmitPassword)

	err = app.Listen(":3000")
	if err != nil {
		panic(err.Error())
	}
}
