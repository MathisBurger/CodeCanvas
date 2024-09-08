package models

import "gorm.io/gorm"

type User struct {
	gorm.Model
	Username   string   `gorm:"unique" json:"username"`
	Password   string   `json:"-"`
	Email      string   `gorm:"-" json:"email"`
	Roles      []string `gorm:"-" json:"roles"`
	ResetToken *string  `gorm:"-"`
}
