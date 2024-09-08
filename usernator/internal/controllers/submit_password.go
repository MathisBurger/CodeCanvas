package controllers

import (
	"github.com/gofiber/fiber/v2"
	"golang.org/x/crypto/bcrypt"
	"usernator/internal/models"
	"usernator/internal/shared"
)

type SubmitPasswordRequest struct {
	Password string `json:"password"`
}

func SubmitPassword(ctx *fiber.Ctx) error {

	resetToken := ctx.Query("reset_token", "-")
	req := new(SubmitPasswordRequest)
	if err := ctx.BodyParser(req); err != nil {
		return fiber.NewError(fiber.StatusBadRequest, err.Error())
	}
	var user models.User
	shared.Database.Find(&user, "reset_token = ?", resetToken)
	if user.ID == 0 {
		return fiber.ErrUnauthorized
	}
	hash, err := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	if err != nil {
		return fiber.NewError(fiber.StatusInternalServerError, err.Error())
	}
	shared.Database.Model(&user).Update("Password", string(hash))
	return ctx.JSON(user)
}
