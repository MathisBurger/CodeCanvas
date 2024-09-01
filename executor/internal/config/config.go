package config

type Configuration struct {
	RabbitMQ MessagingConfig `json:"rabbitmq"`
}

type MessagingConfig struct {
	Username string `json:"username"`
	Password string `json:"password"`
	Host     string `json:"host"`
}
