package services

import (
	"fmt"
	"github.com/runabol/tork"
	"github.com/runabol/tork/engine"
	"github.com/runabol/tork/input"
	"github.com/runabol/tork/middleware/web"
)

func ExecuteTask(c web.Context, task input.Task) (*tork.Job, error) {

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
	job, err := engine.SubmitJob(c.Request().Context(), input, listener)
	if err != nil {
		return nil, err
	}
	return job, nil
}

func handleExecution(result chan string) {
	res := <-result
	fmt.Println(res)
}
