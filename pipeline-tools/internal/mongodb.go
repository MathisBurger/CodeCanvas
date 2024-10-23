package internal

import (
	"context"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

func InitMongoDb() {
	uri := "mongodb://admin:admin@127.0.0.1:27017/admin?retryWrites=true&w=majority"
	client, err := mongo.Connect(context.Background(), options.Client().ApplyURI(uri))
	if err != nil {
		panic(err)
	}
	db := client.Database("executor_files")
	err = db.CreateCollection(context.Background(), "task_files")
	if err != nil {
		panic(err)
	}
	err = db.CreateCollection(context.Background(), "test_files")
	if err != nil {
		panic(err)
	}
}
