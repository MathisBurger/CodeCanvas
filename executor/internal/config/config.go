package config

type Configuration struct {
	RabbitMQ MessagingConfig `json:"rabbitmq"`
	Mongo    MongoConfig     `json:"mongo"`
}

type MessagingConfig struct {
	Username string `json:"username" env:"RABBITMQ_USERNAME"`
	Password string `json:"password" env:"RABBITMQ_PASSWORD"`
	Host     string `json:"host" env:"RABBITMQ_HOST"`
}

type MongoConfig struct {
	Username string `json:"username" env:"MONGODB_USERNAME"`
	Password string `json:"password" env:"MONGODB_PASSWORD"`
	Host     string `json:"host" env:"MONGODB_HOST"`
	Database string `json:"database" env:"MONGODB_DATABASE"`
}
