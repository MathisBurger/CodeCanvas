package controllers

import (
	"github.com/gofiber/fiber/v2"
	"golang.org/x/crypto/bcrypt"
	"time"
	"usernator/internal/models"
	"usernator/internal/shared"
)

type LoginRequest struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

const (
	maxLoginAttempts = 5
)

var (
	loginAttempts map[string]int
)

func LoginAttemptReset() {
	for {
		loginAttempts = make(map[string]int)
		time.Sleep(3 * time.Hour)
	}
}

func LoginUser(ctx *fiber.Ctx) error {

	req := new(LoginRequest)
	if err := ctx.BodyParser(req); err != nil {
		return fiber.NewError(fiber.StatusBadRequest, err.Error())
	}
	if loginAttempts[req.Username] >= maxLoginAttempts {
		return fiber.NewError(fiber.StatusTooManyRequests, "too many loginAttempts")
	}
	var user models.User
	shared.Database.First(&user, "username = ?", req.Username)
	if user.Username == "" {
		return fiber.NewError(fiber.StatusNotFound, "user not found")
	}
	if err := bcrypt.CompareHashAndPassword([]byte(user.Password), []byte(req.Password)); err != nil {
		loginAttempts[user.Username] = loginAttempts[user.Username] + 1
		return fiber.NewError(fiber.StatusBadRequest, "invalid password")
	}
	return ctx.JSON(user)
}
