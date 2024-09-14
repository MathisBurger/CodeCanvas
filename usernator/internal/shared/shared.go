package shared

import (
	"github.com/rabbitmq/amqp091-go"
	"gorm.io/gorm"
	"usernator/internal/config"
)

var (
	Config   *config.Configuration
	Database *gorm.DB
	RabbitMQ *amqp091.Channel
)
