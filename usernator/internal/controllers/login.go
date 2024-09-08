package controllers

import (
	"github.com/gofiber/fiber/v2"
	"golang.org/x/crypto/bcrypt"
	"usernator/internal/models"
	"usernator/internal/shared"
)

type LoginRequest struct {
	Password string `json:"password"`
}

func LoginUser(ctx *fiber.Ctx) error {

	username := ctx.Params("username")
	req := new(LoginRequest)
	if err := ctx.BodyParser(req); err != nil {
		return fiber.NewError(fiber.StatusBadRequest, err.Error())
	}
	var user models.User
	shared.Database.First(&user, "username = ?", username)
	if user.Username == "" {
		return fiber.NewError(fiber.StatusNotFound, "user not found")
	}
	if err := bcrypt.CompareHashAndPassword([]byte(user.Password), []byte(req.Password)); err != nil {
		return fiber.NewError(fiber.StatusBadRequest, "invalid password")
	}
	return ctx.JSON(user)
}
