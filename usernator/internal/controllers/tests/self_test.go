package tests

import (
	"net/http"
	"net/http/httptest"
	"testing"
	"usernator/internal/server"
)

func TestSelfAsNonUser(t *testing.T) {
	app := server.CreateServer("../../../config.json")

	req := httptest.NewRequest(http.MethodGet, "/self", nil)
	resp, _ := app.Test(req, 1000)
	if resp.StatusCode != http.StatusUnauthorized {
		t.Errorf("got status %d, want 401", resp.StatusCode)
	}
}

func TestSelfAsUser(t *testing.T) {
	app := server.CreateServer("../../../config.json")

	req := httptest.NewRequest(http.MethodGet, "/self", nil)
	req.Header.Set("X-CodeCanvas-UserId", "1")
	req.Header.Set("X-CodeCanvas-UserRoles", "ROLE_ADMIN")
	resp, err := app.Test(req, 1000)
	if err != nil {
		t.Fatal(err)
	}
	if resp.StatusCode != http.StatusOK {
		t.Errorf("got status %d, want 200", resp.StatusCode)
	}
}

func BenchmarkSelfAsUser(b *testing.B) {
	app := server.CreateServer("../../../config.json")
	req := httptest.NewRequest(http.MethodGet, "/self", nil)
	req.Header.Set("X-CodeCanvas-UserId", "1")
	for i := 0; i < b.N; i++ {
		_, err := app.Test(req, 1000)
		if err != nil {
			panic(err)
		}
	}
}
