package messaging

import (
	"context"
	"encoding/json"
	"executor/internal/global"
	"github.com/runabol/tork"
	"strconv"
	"time"
)

type createUser struct {
	Id uint64 `json:"id"`
}

func CreateCreateUserHandler(rmqChan chan bool) {
	<-rmqChan
	if _, err := global.RabbitMQ.QueueDeclare("executor_create_user", false, true, true, false, nil); err != nil {
		panic(err.Error())
	}

	if err := global.RabbitMQ.QueueBind("executor_create_user", "", "global_create_user", false, nil); err != nil {
		panic(err.Error())
	}
	msgs, err := global.RabbitMQ.Consume("executor_create_user", "", true, false, false, false, nil)
	if err != nil {
		return
	}
	for {
		msg := <-msgs
		content := &createUser{}
		err = json.Unmarshal(msg.Body, content)
		if err != nil {
			continue
		}
		now := time.Now()
		user := tork.User{
			ID:           strconv.FormatUint(content.Id, 10),
			Username:     strconv.FormatUint(content.Id, 10),
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
