package grpc

import (
	"context"
	"testing"
	"usernator/api"
	config2 "usernator/internal/config"
	"usernator/internal/shared"
	"usernator/internal/startup"
)

func TestGetUser(t *testing.T) {
	config, _ := config2.LoadConfiguration("../../config.json")
	shared.Config = config
	startup.Database()
	server := &GrpcServer{}
	user, err := server.GetUser(context.Background(), &api.UserRequest{
		UserId: uint64(1),
	})
	if err != nil {
		t.Fatal(err)
	}
	t.Log(user)
}

func TestGetUsers(t *testing.T) {
	config, _ := config2.LoadConfiguration("../../config.json")
	shared.Config = config
	startup.Database()
	server := &GrpcServer{}
	user, err := server.GetUsers(context.Background(), &api.UsersRequest{
		UserIds: []uint64{uint64(1)},
	})
	if err != nil {
		t.Fatal(err)
	}
	if len(user.Users) != 1 {
		t.Fatal("Invalid length")
	}
}
