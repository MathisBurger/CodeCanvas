package grpc

import (
	"context"
	"testing"
	"usernator/api"
)

func TestGetUser(t *testing.T) {
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
