package grpc

import (
	"context"
	"usernator/api"
	"usernator/internal/models"
	"usernator/internal/shared"
)

func (s *GrpcServer) GetUser(ctx context.Context, in *api.UserRequest) (*api.UserResponse, error) {
	var user models.User
	shared.Database.First(&user, "id = ?", in.UserId)
	if user.Username == "" {
		return nil, nil
	}
	return &api.UserResponse{
		Id:       uint64(user.ID),
		Username: user.Username,
		Email:    user.Email,
		Roles:    user.Roles,
	}, nil
}
