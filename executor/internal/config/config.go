package config

type Configuration struct {
	RabbitMQ MessagingConfig `json:"rabbitmq"`
}

type MessagingConfig struct {
	Username string `json:"username" env:"RABBITMQ_USERNAME"`
	Password string `json:"password" env:"RABBITMQ_PASSWORD"`
	Host     string `json:"host" env:"RABBITMQ_HOST"`
}
