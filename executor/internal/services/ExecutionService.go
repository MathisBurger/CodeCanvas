package services

import (
	"errors"
	"fmt"
	"github.com/runabol/tork"
	"github.com/runabol/tork/engine"
	"github.com/runabol/tork/input"
	"github.com/runabol/tork/middleware/web"
)

type ExecRequest struct {
	Code     string `json:"code"`
	Language string `json:"language"`
}

func BuildTask(er ExecRequest) (input.Task, error) {
	var image string
	var run string
	var filename string

	switch er.Language {
	case "":
		return input.Task{}, errors.New("invalid language")
	case "python":
		image = "python:3"
		filename = "script.py"
		run = "python script.py > $TORK_OUTPUT"
	case "go":
		image = "golang:1.19"
		filename = "main.go"
		run = "go run main.go > $TORK_OUTPUT"
	case "bash":
		image = "alpine:3.18.3"
		filename = "script"
		run = "sh ./script > $TORK_OUTPUT"
	default:
		return input.Task{}, errors.New("invalid language")
	}

	return input.Task{
		Name:  "execute code",
		Image: image,
		Run:   run,
		Limits: &input.Limits{
			CPUs:   ".5",  // no more than half a CPU
			Memory: "20m", // no more than 20MB of RAM
		},
		Timeout:  "60s",            // terminate container after 5 seconds
		Networks: []string{"none"}, // disable networking
		Files: map[string]string{
			filename: er.Code,
		}}, nil
}

func ExecuteTask(c web.Context, task input.Task) error {

	input := &input.Job{
		Name:  "code execution",
		Tasks: []input.Task{task},
	}
	result := make(chan string)

	listener := func(j *tork.Job) {
		if j.State == tork.JobStateCompleted {
			result <- j.Execution[0].Result
		} else {
			result <- j.Execution[0].Error
		}
	}

	go handleExecution(result)

	// pass the listener to the submit job call
	_, err := engine.SubmitJob(c.Request().Context(), input, listener)
	if err != nil {
		return err
	}
	return nil
}

func handleExecution(result chan string) {
	res := <-result
	fmt.Println(res)
}
