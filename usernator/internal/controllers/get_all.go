package controllers

import (
	"fmt"
	"github.com/gofiber/fiber/v2"
	"usernator/internal/models"
	"usernator/internal/shared"
	"usernator/pkg"
)

func GetAllStudents(ctx *fiber.Ctx) error {
	currentUser := ctx.Locals("currentUser").(*models.User)
	fmt.Println(currentUser)
	if currentUser == nil || (!pkg.ContainsString(currentUser.Roles, "ROLE_TUTOR") && !pkg.ContainsString(currentUser.Roles, "ROLE_ADMIN")) {
		return fiber.NewError(fiber.StatusUnauthorized, "You need to be authorized")
	}
	var users []models.User
	shared.Database.Raw("SELECT * FROM users WHERE roles @> ARRAY['ROLE_STUDENT']").Scan(&users)
	return ctx.JSON(fiber.Map{
		"students": users,
	})
}

func GetAllTutors(ctx *fiber.Ctx) error {

	currentUser := ctx.Locals("currentUser").(*models.User)

	if currentUser == nil {
		return fiber.NewError(fiber.StatusUnauthorized, "You need to be authorized")
	}
	var users []models.User
	shared.Database.Raw("SELECT * FROM users WHERE roles @> ARRAY['ROLE_TUTOR']").Scan(&users)
	return ctx.JSON(fiber.Map{
		"tutors": users,
	})
}
