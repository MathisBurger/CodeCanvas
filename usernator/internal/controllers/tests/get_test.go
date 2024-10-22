package tests

import (
	"net/http"
	"net/http/httptest"
	"strconv"
	"testing"
	"usernator/internal/models"
	"usernator/internal/server"
	"usernator/internal/shared"
)

func TestGetKnownUser(t *testing.T) {
	app := server.CreateServer("../../../config.json")
	var user models.User
	shared.Database.First(&user)

	req := httptest.NewRequest(http.MethodGet, "/user/"+strconv.Itoa(int(user.ID)), nil)
	resp, err := app.Test(req, 1000)
	if err != nil {
		t.Fatal(err)
	}
	if resp.StatusCode != http.StatusOK {
		t.Errorf("want %d; got %d", http.StatusOK, resp.StatusCode)
	}
}

func TestGetUnknownUser(t *testing.T) {
	app := server.CreateServer("../../../config.json")

	req := httptest.NewRequest(http.MethodGet, "/user/-1", nil)
	resp, err := app.Test(req, 1000)
	if err != nil {
		t.Fatal(err)
	}
	if resp.StatusCode != http.StatusNotFound {
		t.Errorf("want %d; got %d", http.StatusNotFound, resp.StatusCode)
	}
}

func TestGetUserWithNonNumberId(t *testing.T) {
	app := server.CreateServer("../../../config.json")

	req := httptest.NewRequest(http.MethodGet, "/user/ok", nil)
	resp, err := app.Test(req, 1000)
	if err != nil {
		t.Fatal(err)
	}
	if resp.StatusCode != http.StatusBadRequest {
		t.Errorf("want %d; got %d", http.StatusBadRequest, resp.StatusCode)
	}
}
