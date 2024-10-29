package main

import (
	"executor/internal"
	"executor/internal/global"
	"executor/internal/handler"
	"executor/tasky_grpc"
	"fmt"
	"github.com/runabol/tork/cli"
	"github.com/runabol/tork/conf"
	"github.com/runabol/tork/datastore"
	"github.com/runabol/tork/engine"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"os"
	"time"
)

func main() {
	if err := conf.LoadConfig(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	engine.OnDatastoreInit(func(ds datastore.Datastore) error {
		global.Postgres = &ds
		return nil
	})

	appConfig := internal.LoadConfig()
	internal.InitMongoDB(appConfig)

	time.Sleep(time.Second * 8)
	conn, err := grpc.NewClient(appConfig.TaskyGrpcAddr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		panic(err.Error())
	}
	client := tasky_grpc.NewTaskyApiClient(conn)

	global.Tasky = &client

	engine.RegisterEndpoint("POST", "/execute", handler.ExecuteHandler)
	// Start the Tork CLI
	app := cli.New()

	if err := app.Run(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
