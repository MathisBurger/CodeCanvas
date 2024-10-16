package middleware

import (
	"github.com/gofiber/fiber/v2"
	"strconv"
	"usernator/internal/models"
	"usernator/internal/shared"
)

func NewAuthMiddleware() fiber.Handler {
	return func(ctx *fiber.Ctx) error {
		userIdString := ctx.Get("X-CodeCanvas-UserId", "-1")
		userId, err := strconv.Atoi(userIdString)
		if err != nil {
			ctx.Locals("currentUser", nil)
			return ctx.Next()
		}
		if userId == -1 || userId == 0 {
			ctx.Locals("currentUser", nil)
		} else {
			var user models.User
			shared.Database.First(&user, "id = ?", userId)
			if user.Username == "" {
				ctx.Locals("currentUser", nil)
			} else {
				ctx.Locals("currentUser", &user)
			}
		}
		return ctx.Next()
	}
}
