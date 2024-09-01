package startup

import (
	"fmt"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"usernator/internal/models"
	"usernator/internal/shared"
)

func Database() {
	dsn := fmt.Sprintf("postgresql://%v:%v@%v/%v?sslmode=disable", shared.Config.Database.Username, shared.Config.Database.Password, shared.Config.Database.Host, shared.Config.Database.Database)
	db, err := gorm.Open(postgres.Open(dsn))
	if err != nil {
		panic(err)
	}
	shared.Database = db
	err = db.AutoMigrate(&models.User{})
	if err != nil {
		panic(err.Error())
	}
}
