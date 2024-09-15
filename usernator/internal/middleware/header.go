package middleware

import (
	"github.com/gofiber/fiber/v2"
	"strconv"
	"strings"
)

func NewAuthMiddleware() fiber.Handler {
	return func(ctx *fiber.Ctx) error {
		userIdString := ctx.Get("X-CodeCanvas-UserId", "-1")
		userId, err := strconv.Atoi(userIdString)
		if err != nil {
			ctx.Locals("userId", -1)
		}
		if userId == -1 || userId == 0 {
			ctx.Locals("userId", -1)
		} else {
			ctx.Locals("userId", userId)
		}
		rolesString := ctx.Get("X-CodeCanvas-UserRoles", "")
		if rolesString == "" {
			ctx.Locals("userRoles", make([]string, 0))
		} else {
			ctx.Locals("userRoles", strings.Split(rolesString, ";"))
		}
		return ctx.Next()
	}
}
