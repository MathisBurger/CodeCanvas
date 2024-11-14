package server

import (
	"github.com/gofiber/fiber/v2"
	"os"
	"usernator/internal/config"
	"usernator/internal/controllers"
	"usernator/internal/grpc"
	"usernator/internal/middleware"
	"usernator/internal/shared"
	"usernator/internal/startup"
)

func CreateServer(configPath string) *fiber.App {
	go controllers.LoginAttemptReset()
	conf, err := config.LoadConfiguration(configPath)
	if err != nil {
		panic(err.Error())
	}
	shared.Config = conf
	startup.Database()
	if os.Getenv("TEST_MODE") != "true" {
		go grpc.StartGrpcServer()
	}
	startup.InitTaskyGrpcClient()

	app := fiber.New(fiber.Config{})

	app.Use(middleware.NewAuthMiddleware())

	app.Get("/self", controllers.GetSelf)
	app.Get("/user/:id", controllers.GetUser)
	app.Get("/all-students", controllers.GetAllStudents)
	app.Get("/all-tutors", controllers.GetAllTutors)
	app.Post("/register", controllers.RegisterUser)
	app.Post("/login", controllers.LoginUser)
	app.Post("/create_tutor", controllers.CreateTutor)
	app.Post("/switch_tutor", controllers.SwitchToTutor)

	// These endpoints are currently disabled and therefore not accessed
	//app.Post("/reset_password", controllers.ResetPassword)
	//app.Post("/submit_password", controllers.SubmitPassword)

	return app
}
