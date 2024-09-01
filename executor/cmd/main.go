package main

import (
	"executor/internal"
	"executor/internal/global"
	"executor/internal/handler"
	"executor/internal/messaging"
	"fmt"
	"github.com/runabol/tork/cli"
	"github.com/runabol/tork/conf"
	"github.com/runabol/tork/datastore"
	"github.com/runabol/tork/engine"
	"os"
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
	internal.InitRabbitMQ(appConfig)

	messaging.CreateCreateUserHandler()

	engine.RegisterEndpoint("POST", "/execute", handler.ExecuteHandler)

	// Start the Tork CLI
	app := cli.New()

	if err := app.Run(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
