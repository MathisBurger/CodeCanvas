package services

import (
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

	job, err := engine.SubmitJob(c.Request().Context(), input)
	if err != nil {
		return nil, err
	}
	return job, nil
}
