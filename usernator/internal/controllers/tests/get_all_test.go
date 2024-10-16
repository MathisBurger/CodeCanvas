package tests

import (
	"net/http"
	"net/http/httptest"
	"testing"
	"usernator/internal/server"
)

func TestGetAllStudentsUnauthorized(t *testing.T) {
	app := server.CreateServer("../../../config.json")
	req := httptest.NewRequest(http.MethodGet, "/all-students", nil)
	resp, _ := app.Test(req, 1000)
	if resp.StatusCode != http.StatusUnauthorized {
		t.Errorf("got status code %d, want %d", resp.StatusCode, http.StatusUnauthorized)
	}
}

func TestGetAllStudentsAsAdmin(t *testing.T) {
	app := server.CreateServer("../../../config.json")
	req := httptest.NewRequest(http.MethodGet, "/all-students", nil)
	req.Header.Set("X-CodeCanvas-UserId", "1")
	req.Header.Set("X-CodeCanvas-UserRoles", "ROLE_ADMIN")
	resp, _ := app.Test(req, 1000)
	if resp.StatusCode != http.StatusOK {
		t.Errorf("got status code %d, want %d", resp.StatusCode, http.StatusOK)
	}
}

func TestGetAllStudentsAsTutor(t *testing.T) {
	app := server.CreateServer("../../../config.json")
	req := httptest.NewRequest(http.MethodGet, "/all-students", nil)
	req.Header.Set("X-CodeCanvas-UserId", "2")
	req.Header.Set("X-CodeCanvas-UserRoles", "ROLE_TUTOR")
	resp, _ := app.Test(req, 1000)
	if resp.StatusCode != http.StatusOK {
		t.Errorf("got status code %d, want %d", resp.StatusCode, http.StatusOK)
	}
}

func TestGetAllTutorsAsStudent(t *testing.T) {
	app := server.CreateServer("../../../config.json")
	req := httptest.NewRequest(http.MethodGet, "/all-tutors", nil)
	req.Header.Set("X-CodeCanvas-UserId", "3")
	req.Header.Set("X-CodeCanvas-UserRoles", "ROLE_STUDENT")
	resp, _ := app.Test(req, 1000)
	if resp.StatusCode != http.StatusOK {
		t.Errorf("got status code %d, want %d", resp.StatusCode, http.StatusOK)
	}
}

func TestGetAllTutorsAsTutor(t *testing.T) {
	app := server.CreateServer("../../../config.json")
	req := httptest.NewRequest(http.MethodGet, "/all-tutors", nil)
	req.Header.Set("X-CodeCanvas-UserId", "2")
	req.Header.Set("X-CodeCanvas-UserRoles", "ROLE_TUTOR")
	resp, _ := app.Test(req, 1000)
	if resp.StatusCode != http.StatusOK {
		t.Errorf("got status code %d, want %d", resp.StatusCode, http.StatusOK)
	}
}

func TestGetAllTutorsAsAdmin(t *testing.T) {
	app := server.CreateServer("../../../config.json")
	req := httptest.NewRequest(http.MethodGet, "/all-tutors", nil)
	req.Header.Set("X-CodeCanvas-UserId", "1")
	req.Header.Set("X-CodeCanvas-UserRoles", "ROLE_ADMIN")
	resp, _ := app.Test(req, 1000)
	if resp.StatusCode != http.StatusOK {
		t.Errorf("got status code %d, want %d", resp.StatusCode, http.StatusOK)
	}
}
