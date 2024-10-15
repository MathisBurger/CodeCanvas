package internal

import "github.com/rabbitmq/amqp091-go"

// / Initializes rabbitmq exchange topic
func InitRabbitMQ() {
	channel := getRabbitMqConnection()
	err := channel.ExchangeDeclare("global_create_user", "topic", true, false, false, false, nil)
	if err != nil {
		panic(err)
	}
}

func getRabbitMqConnection() *amqp091.Channel {
	conn, err := amqp091.Dial("amqp://guest:guest@127.0.0.1:5672")
	if err != nil {
		panic(err.Error())
	}
	ch, err := conn.Channel()
	if err != nil {
		panic(err.Error())
	}
	return ch
}
