package tests

import (
	"bytes"
	"net/http"
	"testing"
	"usernator/internal/server"
)

func TestLogin(t *testing.T) {
	app := server.CreateServer("../../../config.json")
	jsonBody := []byte(`{"username":"admin","password":"123"}`)
	bodyReader := bytes.NewReader(jsonBody)
	req, _ := http.NewRequest("POST", "/login", bodyReader)
	req.Header.Set("Content-Type", "application/json")
	resp, _ := app.Test(req, 1000)
	if resp.StatusCode != http.StatusOK {
		body := make([]byte, resp.ContentLength)
		_, _ = resp.Body.Read(body)
		t.Errorf("handler returned wrong status code: got %v want %v. Error message: %s", resp.StatusCode, http.StatusOK, string(body))
	}
}

func TestLoginWithBadPassword(t *testing.T) {
	app := server.CreateServer("../../../config.json")
	jsonBody := []byte(`{"username":"admin","password":"123456"}`)
	bodyReader := bytes.NewReader(jsonBody)
	req, _ := http.NewRequest("POST", "/login", bodyReader)
	req.Header.Set("Content-Type", "application/json")
	resp, _ := app.Test(req, 1000)
	if resp.StatusCode != http.StatusBadRequest {
		t.Errorf("handler returned wrong status code: got %v want %v", resp.StatusCode, http.StatusBadRequest)
	}
}

func TestLoginAsNonExistingUser(t *testing.T) {
	app := server.CreateServer("../../../config.json")
	jsonBody := []byte(`{"username":"admin13","password":"123456"}`)
	bodyReader := bytes.NewReader(jsonBody)
	req, _ := http.NewRequest("POST", "/login", bodyReader)
	req.Header.Set("Content-Type", "application/json")
	resp, _ := app.Test(req, 1000)
	if resp.StatusCode != http.StatusNotFound {
		t.Errorf("handler returned wrong status code: got %v want %v", resp.StatusCode, http.StatusNotFound)
	}
}
