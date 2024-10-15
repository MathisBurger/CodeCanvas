package middleware

import (
	"github.com/gofiber/fiber/v2"
	"testing"
	config2 "usernator/internal/config"
	"usernator/internal/models"
	"usernator/internal/shared"
	"usernator/internal/startup"
)

func TestMiddlewareAuthFlow(t *testing.T) {
	config, _ := config2.LoadConfiguration("../../config.json")
	shared.Config = config
	startup.Database()
	mw := NewAuthMiddleware()
	ctx := fiber.Ctx{}
	ctx.Request().Header.Set("X-CodeCanvas-UserId", "1")
	ctx.Request().Header.Set("X-CodeCanvas-UserRoles", "ROLE_ADMIN")
	err := mw(&ctx)
	if err != nil {
		t.Errorf("Error: %v", err)
		return
	}
	user := ctx.Locals("currentUser").(*models.User)
	if user == nil {
		t.Errorf("Error: User should not be nil")
		return
	}
	if user.ID != 1 {
		t.Errorf("Error: User id should be 1")
	}
}

func TestMiddlewareAuthFlow2(t *testing.T) {
	config, _ := config2.LoadConfiguration("../../config.json")
	shared.Config = config
	startup.Database()
	mw := NewAuthMiddleware()
	ctx := fiber.Ctx{}
	ctx.Request().Header.Set("X-CodeCanvas-UserId", "1")
	err := mw(&ctx)
	if err != nil {
		t.Errorf("Error: %v", err)
		return
	}
	user := ctx.Locals("currentUser").(*models.User)
	if user == nil {
		t.Errorf("Error: User should not be nil")
		return
	}
	if user.ID != 1 {
		t.Errorf("Error: User id should be 1")
	}
}

func TestMiddlewareAuthFlow3(t *testing.T) {
	config, _ := config2.LoadConfiguration("../../config.json")
	shared.Config = config
	startup.Database()
	mw := NewAuthMiddleware()
	ctx := fiber.Ctx{}
	ctx.Request().Header.Set("X-CodeCanvas-UserId", "-1")
	err := mw(&ctx)
	if err != nil {
		t.Errorf("Error: %v", err)
		return
	}
	user := ctx.Locals("currentUser").(*models.User)
	if user != nil {
		t.Errorf("Error: User should be nil")
	}
}
