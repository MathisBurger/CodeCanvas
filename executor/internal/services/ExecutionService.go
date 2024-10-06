package services

import (
	"context"
	"github.com/runabol/tork"
	"github.com/runabol/tork/engine"
	"github.com/runabol/tork/input"
	"github.com/runabol/tork/middleware/web"
)

func ExecuteTask(c web.Context, task input.Task, username string) (*tork.Job, error) {

	input := &input.Job{
		Name:  task.Name,
		Tasks: []input.Task{task},
	}

	// pass the listener to the submit job call
	ctx := context.WithValue(c.Request().Context(), "username", username)
	job, err := engine.SubmitJob(ctx, input)
	if err != nil {
		return nil, err
	}
	return job, nil
}
