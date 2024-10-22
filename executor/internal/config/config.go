package config

type Configuration struct {
	Mongo MongoConfig `json:"mongo"`
}

type MongoConfig struct {
	Username string `json:"username" env:"MONGODB_USERNAME"`
	Password string `json:"password" env:"MONGODB_PASSWORD"`
	Host     string `json:"host" env:"MONGODB_HOST"`
	Database string `json:"database" env:"MONGODB_DATABASE"`
}
