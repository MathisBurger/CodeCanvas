package handler

import (
	"executor/internal/services"
	"github.com/runabol/tork/middleware/web"
	"net/http"
)

type jobResponse struct {
	Id string `json:"id"`
}

func ExecuteHandler(c web.Context) error {
	req := services.ExecRequest{}
	if err := c.Bind(&req); err != nil {
		c.Error(http.StatusBadRequest, err)
		return nil
	}

	task, err := services.BuildTask(req)
	if err != nil {
		c.Error(http.StatusBadRequest, err)
	}
	job, err := services.ExecuteTask(c, task, req.Solution.Id)
	if err != nil {
		c.Error(http.StatusInternalServerError, err)
	}
	return c.JSON(http.StatusOK, jobResponse{
		Id: job.ID,
	})
}
