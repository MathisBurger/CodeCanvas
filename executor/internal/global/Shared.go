package global

import (
	"executor/tasky_grpc"
	"github.com/runabol/tork/datastore"
	"go.mongodb.org/mongo-driver/mongo"
)

var (
	Postgres *datastore.Datastore
	MongoDB  *mongo.Database
	Tasky    *tasky_grpc.TaskyApiClient
)
