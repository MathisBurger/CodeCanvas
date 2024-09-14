package messaging

import (
	"context"
	"encoding/json"
	"executor/internal/global"
	"github.com/runabol/tork"
	"time"
)

type createUser struct {
	Id string `json:"id"`
}

func CreateCreateUserHandler() {

	err := global.RabbitMQ.ExchangeDeclare(
		"global_create_user", // name
		"topic",              // type
		true,                 // durable
		false,                // auto-deleted
		false,                // internal
		false,                // no-wait
		nil,                  // arguments
	)
	if err != nil {
		panic(err)
	}
	if _, err := global.RabbitMQ.QueueDeclare("executor_create_user", false, true, true, false, nil); err != nil {
		panic(err.Error())
	}

	if err := global.RabbitMQ.QueueBind("executor_create_user", "", "global_create_user", false, nil); err != nil {
		panic(err.Error())
	}
	msgs, err := global.RabbitMQ.Consume("executor_create_user", "", true, false, false, false, nil)
	if err != nil {
		panic(err.Error())
	}
	for msg := range msgs {
		content := &createUser{}
		err = json.Unmarshal(msg.Body, content)
		if err != nil {
			continue
		}
		now := time.Now()
		user := tork.User{
			ID:           content.Id,
			Username:     content.Id,
			Password:     "",
			PasswordHash: "",
			CreatedAt:    &now,
			Disabled:     false,
		}
		ctx := context.Background()
		err = (*global.Postgres).CreateUser(ctx, &user)
		if err != nil {
			return
		}
	}
}
