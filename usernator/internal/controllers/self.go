package controllers

import (
	"context"
	"github.com/gofiber/fiber/v2"
	"usernator/internal/models"
	"usernator/internal/shared"
	"usernator/tasky_grpc"
)

func GetSelf(ctx *fiber.Ctx) error {
	user, ok := ctx.Locals("currentUser").(*models.User)
	if !ok {
		return fiber.NewError(fiber.StatusUnauthorized, "You need to be logged in")
	}
	if user == nil {
		return fiber.NewError(fiber.StatusNotFound, "User not found")
	}
	rawGroups, err := (*shared.Tasky).GetUserGroups(context.Background(), &tasky_grpc.GroupsRequest{
		UserId: uint64(user.ID),
	})
	if err != nil {
		return fiber.NewError(fiber.StatusInternalServerError, err.Error())
	}
	var groups []models.Group
	for _, group := range (*rawGroups).Groups {
		var tutor models.User
		shared.Database.First(&tutor, "id = ?", group.GetTutorId())
		groups = append(groups, models.Group{
			ID:          uint(group.GetId()),
			Name:        group.GetTitle(),
			MemberCount: uint(group.GetMemberCount()),
			Tutor:       tutor,
		})
	}
	resp := models.SelfUser{
		ID:       user.ID,
		Username: user.Username,
		Email:    user.Email,
		Roles:    user.Roles,
		Groups:   groups,
	}
	return ctx.JSON(resp)
}
