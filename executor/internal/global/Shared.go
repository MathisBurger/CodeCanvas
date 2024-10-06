package global

import (
	"github.com/rabbitmq/amqp091-go"
	"github.com/runabol/tork/datastore"
	"go.mongodb.org/mongo-driver/mongo"
)

var (
	Postgres *datastore.Datastore
	RabbitMQ *amqp091.Channel
	MongoDB  *mongo.Database
)
