package startup

import (
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"os"
	"time"
	"usernator/internal/shared"
	"usernator/tasky_grpc"
)

func InitTaskyGrpcClient() {
	// Waiting for tasky to start can be skipped in test mode,
	// because container is already online for some time
	if os.Getenv("TEST_MODE") != "true" {
		time.Sleep(8 * time.Second)
	}
	conn, err := grpc.NewClient(shared.Config.Messaging.TaskyAddr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		panic(err.Error())
	}
	client := tasky_grpc.NewTaskyApiClient(conn)
	shared.Tasky = &client
}
