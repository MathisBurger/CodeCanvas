package shared

import (
	"gorm.io/gorm"
	"usernator/internal/config"
	"usernator/tasky_grpc"
)

var (
	Config   *config.Configuration
	Database *gorm.DB
	Tasky    *tasky_grpc.TaskyApiClient
)
