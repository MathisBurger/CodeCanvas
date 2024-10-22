package main

import (
	"fmt"
	"golang.org/x/crypto/bcrypt"
	config2 "usernator/internal/config"
	"usernator/internal/models"
	"usernator/internal/shared"
	"usernator/internal/startup"
)

func main() {
	config, _ := config2.LoadConfiguration("./config.json")
	shared.Config = config
	startup.Database()

	password, _ := bcrypt.GenerateFromPassword([]byte("123"), bcrypt.DefaultCost)

	var admin models.User
	admin.Username = "admin"
	admin.Password = string(password)
	admin.Roles = []string{"ROLE_ADMIN"}

	var tutor models.User
	tutor.Username = "tutor"
	tutor.Password = string(password)
	tutor.Roles = []string{"ROLE_TUTOR"}

	var student1 models.User
	student1.Username = "student1"
	student1.Password = string(password)
	student1.Roles = []string{"ROLE_STUDENT"}

	var student2 models.User
	student2.Username = "student2"
	student2.Password = string(password)
	student2.Roles = []string{"ROLE_STUDENT"}

	var student3 models.User
	student3.Username = "student3"
	student3.Password = string(password)
	student3.Roles = []string{"ROLE_STUDENT"}

	shared.Database.Create(&admin)
	shared.Database.Create(&tutor)
	shared.Database.Create(&student1)
	shared.Database.Create(&student2)
	shared.Database.Create(&student3)
	fmt.Println("Created fixture data")
}
