package services

import (
	"context"
	"fmt"
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
	result := make(chan string)

	listener := func(j *tork.Job) {
		if j.State == tork.JobStateCompleted {
			result <- j.Execution[0].Result
		} else if j.State == tork.JobStateRunning {
			result <- j.Output
		} else {
			result <- j.Execution[0].Error
		}
	}

	go handleExecution(result)

	// pass the listener to the submit job call
	ctx := context.WithValue(c.Request().Context(), "username", username)
	fmt.Println(ctx.Value("username"))
	job, err := engine.SubmitJob(ctx, input, listener)
	if err != nil {
		return nil, err
	}
	return job, nil
}

func handleExecution(result chan string) {
	res := <-result
	fmt.Println(res)
}
