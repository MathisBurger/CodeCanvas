package startup

import (
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"time"
	"usernator/internal/shared"
	"usernator/tasky_grpc"
)

func InitTaskyGrpcClient() {
	// Wait some time in order to start tasky container
	time.Sleep(8 * time.Second)
	conn, err := grpc.NewClient(shared.Config.Messaging.TaskyAddr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		panic(err.Error())
	}
	client := tasky_grpc.NewTaskyApiClient(conn)
	shared.Tasky = &client
}
