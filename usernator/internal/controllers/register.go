package controllers

import (
	"context"
	"encoding/json"
	"github.com/gofiber/fiber/v2"
	"github.com/rabbitmq/amqp091-go"
	"golang.org/x/crypto/bcrypt"
	"time"
	"usernator/internal/models"
	"usernator/internal/shared"
)

type CreateRequest struct {
	Username string `json:"username"`
	Password string `json:"password"`
	Email    string `json:"email"`
}

func RegisterUser(ctx *fiber.Ctx) error {
	req := new(CreateRequest)
	if err := ctx.BodyParser(req); err != nil {
		return err
	}
	var user models.User
	shared.Database.First(&user, "username = ?", req.Username)
	if user.Username != "" {
		return fiber.NewError(fiber.StatusFound, "User already exists")
	}
	user.Username = req.Username
	hash, err := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	if err != nil {
		return fiber.NewError(fiber.StatusInternalServerError, err.Error())
	}
	user.Password = string(hash)
	user.Email = req.Email
	user.ResetToken = nil
	user.Roles = []string{"ROLE_STUDENT"}
	shared.Database.Create(&user)
	rmqctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	data, err := json.Marshal(user)
	if err != nil {
		return fiber.NewError(fiber.StatusInternalServerError, err.Error())
	}
	err = shared.RabbitMQ.PublishWithContext(rmqctx, "global_create_user", "", false, false, amqp091.Publishing{
		ContentType: "application/json",
		Body:        data,
	})
	if err != nil {
		return fiber.NewError(fiber.StatusInternalServerError, err.Error())
	}
	return ctx.JSON(user)
}
