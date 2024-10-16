package server

import (
	"github.com/gofiber/fiber/v2"
	"usernator/internal/config"
	"usernator/internal/controllers"
	"usernator/internal/middleware"
	"usernator/internal/shared"
	"usernator/internal/startup"
)

func CreateServer(configPath string) *fiber.App {
	conf, err := config.LoadConfiguration(configPath)
	if err != nil {
		panic(err.Error())
	}
	shared.Config = conf
	startup.Database()
	go startup.InitTaskyGrpcClient()

	app := fiber.New(fiber.Config{})

	app.Use(middleware.NewAuthMiddleware())

	app.Get("/self", controllers.GetSelf)
	app.Get("/user/:id", controllers.GetUser)
	app.Get("/all-students", controllers.GetAllStudents)
	app.Get("/all-tutors", controllers.GetAllTutors)
	app.Post("/register", controllers.RegisterUser)
	app.Post("/login", controllers.LoginUser)

	// These endpoints are currently disabled and therefore not accessed
	//app.Post("/reset_password", controllers.ResetPassword)
	//app.Post("/submit_password", controllers.SubmitPassword)
	//app.Post("/create_tutor", controllers.CreateTutor)

	return app
}
