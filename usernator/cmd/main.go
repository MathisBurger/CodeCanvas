package main

import (
	"github.com/gofiber/fiber/v2"
	"usernator/internal/config"
	"usernator/internal/controllers"
	"usernator/internal/grpc"
	"usernator/internal/middleware"
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
	go startup.InitRabbitMQ()
	go startup.InitTaskyGrpcClient()

	app := fiber.New(fiber.Config{})

	app.Use(middleware.NewAuthMiddleware())

	app.Get("/self", controllers.GetSelf)
	app.Get("/user/:id", controllers.GetUser)
	app.Get("/all-students", controllers.GetAllStudents)
	app.Get("/all-tutors", controllers.GetAllTutors)
	app.Post("/register", controllers.RegisterUser)
	app.Post("/login", controllers.LoginUser)
	app.Post("/reset_password", controllers.ResetPassword)
	app.Post("/submit_password", controllers.SubmitPassword)
	app.Post("/createTutor", controllers.CreateTutor)

	go grpc.StartGrpcServer()
	err = app.Listen(":3000")
	if err != nil {
		panic(err.Error())
	}
}
