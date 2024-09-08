package controllers

import (
	"github.com/gofiber/fiber/v2"
	"strconv"
	"usernator/internal/models"
	"usernator/internal/shared"
)

func GetUser(ctx *fiber.Ctx) error {

	id := ctx.Params("id")

	idNumber, err := strconv.Atoi(id)
	if err != nil {
		return fiber.NewError(fiber.StatusBadRequest, "The ID has an invalid format")
	}
	var user models.User
	shared.Database.First(&user, "id = ?", idNumber)
	return ctx.JSON(user)
}
