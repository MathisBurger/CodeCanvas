package config

type Configuration struct {
	Mongo         MongoConfig `json:"mongo"`
	TaskyGrpcAddr string      `json:"tasky_grpc_addr" env:"TASKY_GRPC_ADDR"`
}

type MongoConfig struct {
	Username string `json:"username" env:"MONGODB_USERNAME"`
	Password string `json:"password" env:"MONGODB_PASSWORD"`
	Host     string `json:"host" env:"MONGODB_HOST"`
	Database string `json:"database" env:"MONGODB_DATABASE"`
}
