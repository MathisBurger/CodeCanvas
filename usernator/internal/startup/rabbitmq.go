package startup

import (
	"github.com/rabbitmq/amqp091-go"
	"time"
	"usernator/internal/shared"
)

func InitRabbitMQ() {
	time.Sleep(15 * time.Second)
	c := shared.Config
	conn, err := amqp091.Dial("amqp://" + c.Messaging.Username + ":" + c.Messaging.Password + "@" + c.Messaging.Host)
	if err != nil {
		panic(err.Error())
	}
	ch, err := conn.Channel()
	if err != nil {
		panic(err.Error())
	}
	shared.RabbitMQ = ch
}
