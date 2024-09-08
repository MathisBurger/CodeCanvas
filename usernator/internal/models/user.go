package models

import "github.com/lib/pq"

type User struct {
	BaseModel
	Username   string         `gorm:"unique" json:"username"`
	Password   string         `json:"-"`
	Email      string         `gorm:"-" json:"email"`
	Roles      pq.StringArray `gorm:"type:text[]" json:"roles"`
	ResetToken *string        `gorm:"-" json:"resetToken"`
}
