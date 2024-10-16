package config

import (
	"context"
	"encoding/json"
	"github.com/sethvargo/go-envconfig"
	"os"
)

type Configuration struct {
	Database  DatabaseConfiguration `json:"database"`
	Smtp      SmtpConfiguration     `json:"smtp"`
	Template  TemplateConfiguration `json:"template"`
	Messaging MessagingConfig       `json:"messaging"`
}

type DatabaseConfiguration struct {
	Host     string `json:"host" env:"DB_HOST"`
	Port     string `json:"port" env:"DB_PORT"`
	Username string `json:"username" env:"DB_USERNAME"`
	Password string `json:"password" env:"DB_PASSWORD"`
	Database string `json:"database" env:"DB_DATABASE"`
}

type SmtpConfiguration struct {
	Host     string `json:"host" env:"SMTP_HOST"`
	Port     string `json:"port" env:"SMTP_PORT"`
	Username string `json:"username" env:"SMTP_USERNAME"`
	Password string `json:"password" env:"SMTP_PASSWORD"`
}

type MessagingConfig struct {
	TaskyAddr string `json:"tasky_addr" env:"TASKY_ADDR"`
}

type TemplateConfiguration struct {
	BaseUrl string `json:"base_url" env:"TEMPLATE_BASE_URL"`
}

func LoadConfiguration(path string) (*Configuration, error) {
	c := &Configuration{}
	configFile, err := os.Open(path)
	if err != nil {
		ctx := context.Background()
		if err = envconfig.Process(ctx, c); err != nil {
			return nil, err
		}
		return c, nil
	}
	defer configFile.Close()
	jsonParser := json.NewDecoder(configFile)
	err = jsonParser.Decode(c)
	if err != nil {
		return nil, err
	}
	return c, nil
}
