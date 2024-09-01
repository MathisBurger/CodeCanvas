package shared

import (
	"gorm.io/gorm"
	"usernator/internal/config"
)

var (
	Config   *config.Configuration
	Database *gorm.DB
)
