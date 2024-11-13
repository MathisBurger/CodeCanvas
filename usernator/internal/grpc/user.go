package grpc

import (
	"context"
	"strings"
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

func (s *GrpcServer) GetUsers(ctx context.Context, in *api.UsersRequest) (*api.UsersResponse, error) {
	var users []models.User
	shared.Database.Where("id IN ?", in.UserIds).Find(&users)
	var responseUsers []*api.UserResponse
	for _, user := range users {
		responseUsers = append(responseUsers, &api.UserResponse{
			Id:       uint64(user.ID),
			Username: user.Username,
			Email:    user.Email,
			Roles:    user.Roles,
		})
	}
	return &api.UsersResponse{
		Users: responseUsers,
	}, nil
}

func (s *GrpcServer) SearchStudents(ctx context.Context, in *api.SearchStudentsRequest) (*api.UsersResponse, error) {
	var users []models.User
	shared.Database.Where(
		"roles @> ARRAY['ROLE_STUDENT'] AND to_tsvector('english', username) @@ to_tsquery('english', ?)", strings.Join(strings.Split(" ", in.Search), "&")).Find(&users)
	var responseUsers []*api.UserResponse
	for _, user := range users {
		responseUsers = append(responseUsers, &api.UserResponse{
			Id:       uint64(user.ID),
			Username: user.Username,
			Email:    user.Email,
			Roles:    user.Roles,
		})
	}
	return &api.UsersResponse{
		Users: responseUsers,
	}, nil
}
