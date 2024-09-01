package global

import (
	"github.com/rabbitmq/amqp091-go"
	"github.com/runabol/tork/datastore"
)

var (
	Postgres *datastore.Datastore
	RabbitMQ *amqp091.Channel
)
