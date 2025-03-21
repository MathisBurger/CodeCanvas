package services

import (
	"context"
	"executor/internal/global"
	"executor/tasky_grpc"
	"github.com/runabol/tork"
	"github.com/runabol/tork/engine"
	"github.com/runabol/tork/input"
	"github.com/runabol/tork/middleware/web"
	"log"
	"strings"
)

func ExecuteTask(c web.Context, task input.Task, solutionId int) (*tork.Job, error) {

	input := &input.Job{
		Name:  task.Name,
		Tasks: []input.Task{task},
	}

	listener := func(j *tork.Job) {
		if j.State == tork.JobStateCompleted {
			if len(j.Execution) == 0 {
				return
			}
			status := "SUCCESSFUL"
			if strings.HasSuffix(strings.TrimSuffix(j.Execution[0].Result, "\n"), "FAILED") {
				status = "FAILED"
				_ = (*global.Postgres).UpdateJob(context.Background(), j.ID, func(u *tork.Job) error {
					u.State = tork.JobStateFailed
					return nil
				})
			}
			_, err := (*global.Tasky).UpdateSolutionStatus(context.Background(), &tasky_grpc.SolutionUpdateStatusRequest{
				Status:     status,
				SolutionId: uint64(solutionId),
			})
			if err != nil {
				log.Fatal(err.Error())
			}

		} else {
			_, err := (*global.Tasky).UpdateSolutionStatus(context.Background(), &tasky_grpc.SolutionUpdateStatusRequest{
				Status:     "FAILED",
				SolutionId: uint64(solutionId),
			})
			if err != nil {
				log.Fatal(err.Error())
			}
		}
	}

	job, err := engine.SubmitJob(c.Request().Context(), input, listener)
	if err != nil {
		return nil, err
	}

	return job, nil
}
