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
		"roles @> ARRAY['ROLE_STUDENT'] AND username LIKE ?", "%"+in.Search+"%").Limit(30).Find(&users)
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
