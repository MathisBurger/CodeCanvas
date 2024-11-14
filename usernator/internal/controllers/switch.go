package controllers

import (
	"github.com/gofiber/fiber/v2"
	"usernator/internal/models"
	"usernator/internal/shared"
	"usernator/internal/util"
)

// / Endpoint to switch to tutor
func SwitchToTutor(ctx *fiber.Ctx) error {
	currentUser, ok := ctx.Locals("currentUser").(*models.User)
	if !ok {
		return fiber.NewError(fiber.StatusUnauthorized, "You need to be logged in")
	}
	if !util.ArrayContains(currentUser.Roles, "ROLE_STUDENT") {
		return fiber.NewError(fiber.StatusForbidden, "You need to be a student in order to switch to tutor account")
	}

	shared.Database.Exec("UPDATE users SET roles = '{ROLE_STUDENT}' WHERE id = ?", currentUser.ID)
	return ctx.SendStatus(fiber.StatusOK)
}
