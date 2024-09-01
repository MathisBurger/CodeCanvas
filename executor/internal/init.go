package internal

import (
	"encoding/json"
	"executor/internal/config"
	"executor/internal/global"
	"github.com/rabbitmq/amqp091-go"
	"log"
	"os"
)

func LoadConfig() *config.Configuration {
	c := &config.Configuration{}
	configFile, err := os.Open("./config.json")
	defer configFile.Close()
	if err != nil {
		log.Fatal(err)
	}
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
		panic(err)
	}
	defer conn.Close()
	ch, err := conn.Channel()
	if err != nil {
		panic(err)
	}
	defer ch.Close()
	global.RabbitMQ = ch
}
