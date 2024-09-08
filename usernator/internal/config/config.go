package config

import (
	"encoding/json"
	"os"
)

type Configuration struct {
	Database DatabaseConfiguration `json:"database"`
	Smtp     SmtpConfiguration     `json:"smtp"`
	Template TemplateConfiguration `json:"template"`
}

type DatabaseConfiguration struct {
	Host     string `json:"host"`
	Port     string `json:"port"`
	Username string `json:"username"`
	Password string `json:"password"`
	Database string `json:"database"`
}

type SmtpConfiguration struct {
	Host     string `json:"host"`
	Port     string `json:"port"`
	Username string `json:"username"`
	Password string `json:"password"`
}

type TemplateConfiguration struct {
	BaseUrl string `json:"base_url"`
}

func LoadConfiguration() (*Configuration, error) {
	c := &Configuration{}
	configFile, err := os.Open("./config.json")
	defer configFile.Close()
	if err != nil {
		return nil, err
	}
	jsonParser := json.NewDecoder(configFile)
	err = jsonParser.Decode(c)
	if err != nil {
		return nil, err
	}
	return c, nil
}
