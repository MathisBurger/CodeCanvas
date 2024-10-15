package tests

import (
	"bytes"
	"net/http"
	"testing"
	"usernator/internal/server"
)

func TestRegister(t *testing.T) {
	app := server.CreateServer("../../../config.json")
	jsonBody := []byte(`{"username":"test","password":"test", "email": ""}`)
	bodyReader := bytes.NewReader(jsonBody)
	req, err := http.NewRequest("POST", "/register", bodyReader)
	if err != nil {
		t.Fatal(err)
	}
	resp, _ := app.Test(req, 1000)
	if resp.StatusCode != http.StatusOK {
		t.Errorf("status code is %v", resp.StatusCode)
	}
}

func TestRegisterExisting(t *testing.T) {
	app := server.CreateServer("../../../config.json")
	jsonBody := []byte(`{"username":"admin","password":"test", "email": ""}`)
	bodyReader := bytes.NewReader(jsonBody)
	req, err := http.NewRequest("POST", "/register", bodyReader)
	if err != nil {
		t.Fatal(err)
	}
	resp, _ := app.Test(req, 1000)
	if resp.StatusCode != http.StatusFound {
		t.Errorf("status code is %v", resp.StatusCode)
	}
}
