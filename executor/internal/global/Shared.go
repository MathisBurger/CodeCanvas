package global

import (
	"github.com/runabol/tork/datastore"
	"go.mongodb.org/mongo-driver/mongo"
)

var (
	Postgres *datastore.Datastore
	MongoDB  *mongo.Database
)
