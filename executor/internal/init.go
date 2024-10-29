package internal

import (
	"context"
	"encoding/json"
	"executor/internal/config"
	"executor/internal/global"
	"fmt"
	"github.com/sethvargo/go-envconfig"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"log"
	"os"
)

func LoadConfig() *config.Configuration {
	c := &config.Configuration{}
	configFile, err := os.Open("./config.json")
	if err != nil {
		ctx := context.Background()
		if err = envconfig.Process(ctx, c); err != nil {
			panic(err.Error())
		}
		return c
	}
	defer configFile.Close()
	jsonParser := json.NewDecoder(configFile)
	err = jsonParser.Decode(c)
	if err != nil {
		log.Fatal(err)
	}
	return c
}

func InitMongoDB(c *config.Configuration) {
	uri := fmt.Sprintf("mongodb://%s:%s@%s/%s?retryWrites=true&w=majority", c.Mongo.Username, c.Mongo.Password, c.Mongo.Host, c.Mongo.Database)
	client, err := mongo.Connect(context.Background(), options.Client().ApplyURI(uri))
	if err != nil {
		panic(err.Error())
	}
	global.MongoDB = client.Database(c.Mongo.Database)
}
