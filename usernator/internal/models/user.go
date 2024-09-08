package models

import "gorm.io/gorm"

type User struct {
	gorm.Model
	Username   string `gorm:"unique" json:"username"`
	Password   string
	Email      string   `gorm:"-" json:"email"`
	Roles      []string `gorm:"-" json:"roles"`
	ResetToken *string  `gorm:"-" json:"reset_token"`
}
