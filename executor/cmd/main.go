package main

import (
	"executor/internal/handler"
	"fmt"
	"github.com/runabol/tork/cli"
	"github.com/runabol/tork/conf"
	"github.com/runabol/tork/engine"
	"os"
)

func main() {
	if err := conf.LoadConfig(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	engine.RegisterEndpoint("POST", "/execute", handler.ExecuteHandler)

	// Start the Tork CLI
	app := cli.New()

	if err := app.Run(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
