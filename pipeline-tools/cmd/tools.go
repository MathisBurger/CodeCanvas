package main

import (
	"fmt"
	"github.com/MathisBurger/CodeCanvas/pipeline-tools/internal"
)

func main() {
	fmt.Println("Starting to initialize infrastructure...")
	internal.InitRabbitMQ()
	internal.InitPostgres()
	internal.InitMongoDb()
	fmt.Println("Infrastructure initialized")
}
