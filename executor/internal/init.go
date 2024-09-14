package internal

import (
	"context"
	"encoding/json"
	"executor/internal/config"
	"executor/internal/global"
	"github.com/rabbitmq/amqp091-go"
	"github.com/sethvargo/go-envconfig"
	"log"
	"os"
)

func LoadConfig() *config.Configuration {
	c := &config.Configuration{}
	configFile, err := os.Open("./config.json")
	if err != nil {
		ctx := context.Background()
		if err = envconfig.Process(ctx, c); err != nil {
			panic(err.Error())
		}
		return c
	}
	defer configFile.Close()
	jsonParser := json.NewDecoder(configFile)
	err = jsonParser.Decode(c)
	if err != nil {
		log.Fatal(err)
	}
	return c
}

func InitRabbitMQ(c *config.Configuration) {
	conn, err := amqp091.Dial("amqp://" + c.RabbitMQ.Username + ":" + c.RabbitMQ.Password + "@" + c.RabbitMQ.Host)
	if err != nil {
		panic(err.Error())
	}
	ch, err := conn.Channel()
	if err != nil {
		panic(err.Error())
	}
	global.RabbitMQ = ch
}
