package models

import "github.com/lib/pq"

type User struct {
	BaseModel
	Username   string         `gorm:"unique" json:"username"`
	Password   string         `json:"-"`
	Email      string         `gorm:"-" json:"email"`
	Roles      pq.StringArray `gorm:"type:text[]" json:"roles"`
	ResetToken *string        `gorm:"-" json:"-"`
}

type SelfUser struct {
	ID       uint           `json:"id"`
	Username string         `json:"username"`
	Password string         `json:"-"`
	Email    string         `json:"email"`
	Roles    pq.StringArray `json:"roles"`
	Groups   []Group        `json:"groups"`
}

type Group struct {
	ID          uint   `json:"id"`
	Name        string `json:"name"`
	MemberCount uint   `json:"member_count"`
	Tutor       User   `json:"tutor"`
}
