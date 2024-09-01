package handler

import (
	"executor/internal/services"
	"github.com/runabol/tork/middleware/web"
	"net/http"
)

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
	err = services.ExecuteTask(c, task)
	if err != nil {
		c.Error(http.StatusInternalServerError, err)
	}
	return c.JSON(http.StatusOK, req)
}
