package controllers

import (
	"github.com/gofiber/fiber/v2"
	"usernator/internal/models"
	"usernator/internal/shared"
)

func GetSelf(ctx *fiber.Ctx) error {
	userId := ctx.Locals("userId").(*int)
	if userId == nil {
		return fiber.NewError(fiber.StatusNotFound, "User not found")
	}
	var user models.User
	shared.Database.First(&user, "id = ?", *userId)
	return ctx.JSON(user)
}
