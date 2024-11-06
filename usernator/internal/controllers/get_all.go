package controllers

import (
	"github.com/gofiber/fiber/v2"
	"strconv"
	"usernator/internal/models"
	"usernator/internal/shared"
	"usernator/pkg"
)

const PageSize = 50

func GetAllStudents(ctx *fiber.Ctx) error {
	currentUser, ok := ctx.Locals("currentUser").(*models.User)
	if !ok {
		return fiber.NewError(fiber.StatusUnauthorized, "You need to be logged in")
	}
	if currentUser == nil || (!pkg.ContainsString(currentUser.Roles, "ROLE_TUTOR") && !pkg.ContainsString(currentUser.Roles, "ROLE_ADMIN")) {
		return fiber.NewError(fiber.StatusUnauthorized, "You need to be authorized")
	}

	page := ctx.QueryInt("page", 1)
	offset := (page - 1) * PageSize

	var count int
	shared.Database.Raw("SELECT COUNT(*) FROM users WHERE roles @> ARRAY['ROLE_STUDENT']").Scan(&count)

	var users []models.User
	shared.Database.Raw("SELECT * FROM users WHERE roles @> ARRAY['ROLE_STUDENT'] LIMIT " + strconv.Itoa(PageSize) + " OFFSET " + strconv.Itoa(offset) + ";").Scan(&users)
	return ctx.JSON(fiber.Map{
		"total":    count,
		"students": users,
	})
}

func GetAllTutors(ctx *fiber.Ctx) error {

	currentUser, ok := ctx.Locals("currentUser").(*models.User)
	if !ok {
		return fiber.NewError(fiber.StatusUnauthorized, "You need to be logged in")
	}

	if currentUser == nil {
		return fiber.NewError(fiber.StatusUnauthorized, "You need to be authorized")
	}

	page := ctx.QueryInt("page", 1)
	offset := (page - 1) * PageSize

	var count int
	shared.Database.Raw("SELECT COUNT(*) FROM users WHERE roles @> ARRAY['ROLE_TUTOR']").Scan(&count)

	var users []models.User
	shared.Database.Raw("SELECT * FROM users WHERE roles @> ARRAY['ROLE_TUTOR'] LIMIT " + strconv.Itoa(PageSize) + " OFFSET " + strconv.Itoa(offset) + ";").Scan(&users)
	return ctx.JSON(fiber.Map{
		"total":  count,
		"tutors": users,
	})
}
