package models

import "gorm.io/gorm"

type User struct {
	gorm.Model
	Username string `gorm:"unique" json:"username"`
	Password string
	Email    string   `gorm:"unique" json:"email"`
	Roles    []string `gorm:"-" json:"roles"`
}
