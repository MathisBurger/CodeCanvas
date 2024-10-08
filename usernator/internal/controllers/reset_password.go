package controllers

import (
	"github.com/gofiber/fiber/v2"
	"net/smtp"
	"usernator/internal/models"
	"usernator/internal/shared"
	"usernator/internal/util"
)

type ResetRequest struct {
	Username string `json:"username"`
}

func ResetPassword(ctx *fiber.Ctx) error {
	req := new(ResetRequest)
	if err := ctx.BodyParser(req); err != nil {
		return fiber.NewError(fiber.StatusBadRequest, err.Error())
	}
	var user models.User
	shared.Database.First(&user, "username = ?", req.Username)
	if user.Username == "" {
		return fiber.NewError(fiber.StatusNotFound, "user not found")
	}
	token := util.StringWithCharset(64)
	shared.Database.Model(&user).Update("ResetToken", token)
	to := []string{user.Email}
	message := []byte("Reset your password here: " + shared.Config.Template.BaseUrl + "/submit_password?reset_token=" + token)
	auth := smtp.PlainAuth("", shared.Config.Smtp.Username, shared.Config.Smtp.Password, shared.Config.Smtp.Host)
	err := smtp.SendMail(shared.Config.Smtp.Host+":"+shared.Config.Smtp.Port, auth, shared.Config.Smtp.Username, to, message)
	if err != nil {
		return fiber.NewError(fiber.StatusInternalServerError, err.Error())
	}
	return ctx.JSON(fiber.Map{})
}
