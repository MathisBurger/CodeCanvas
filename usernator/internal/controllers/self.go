package controllers

import (
	"github.com/gofiber/fiber/v2"
	"usernator/internal/models"
)

func GetSelf(ctx *fiber.Ctx) error {
	user := ctx.Locals("currentUser").(*models.User)
	if user == nil {
		return fiber.NewError(fiber.StatusNotFound, "User not found")
	}
	return ctx.JSON(*user)
}
