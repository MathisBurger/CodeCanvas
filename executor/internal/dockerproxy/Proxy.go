package dockerproxy

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net"
	"net/http"
	"net/http/httputil"
	"net/url"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

const defaultDockerHost = "unix:///var/run/docker.sock"

// Start launches a local proxy in front of the docker daemon which hardens every
// container created through it (all capabilities dropped, no-new-privileges set)
// and points DOCKER_HOST at it, so the tork docker runtime uses the proxy.
// Must be called before the tork engine initializes its runtime.
func Start() error {
	upstream := os.Getenv("DOCKER_HOST")
	if upstream == "" {
		upstream = defaultDockerHost
	}
	if os.Getenv("DOCKER_TLS_VERIFY") != "" {
		return fmt.Errorf("docker proxy does not support TLS connections to the docker daemon")
	}
	u, err := url.Parse(upstream)
	if err != nil {
		return fmt.Errorf("invalid DOCKER_HOST %q: %w", upstream, err)
	}

	var dial func(ctx context.Context, _, _ string) (net.Conn, error)
	switch u.Scheme {
	case "unix":
		dial = func(ctx context.Context, _, _ string) (net.Conn, error) {
			return (&net.Dialer{}).DialContext(ctx, "unix", u.Path)
		}
	case "tcp":
		dial = func(ctx context.Context, _, _ string) (net.Conn, error) {
			return (&net.Dialer{}).DialContext(ctx, "tcp", u.Host)
		}
	default:
		return fmt.Errorf("unsupported DOCKER_HOST scheme %q", u.Scheme)
	}

	proxy := &httputil.ReverseProxy{
		Rewrite: func(r *httputil.ProxyRequest) {
			r.Out.URL.Scheme = "http"
			r.Out.URL.Host = "docker"
			r.Out.Host = "docker"
		},
		Transport:     &http.Transport{DialContext: dial},
		FlushInterval: -1, // stream logs and image pulls immediately
	}

	dir, err := os.MkdirTemp("", "executor-docker-proxy")
	if err != nil {
		return err
	}
	socket := filepath.Join(dir, "docker.sock")
	listener, err := net.Listen("unix", socket)
	if err != nil {
		return err
	}

	go func() {
		_ = http.Serve(listener, harden(proxy))
	}()

	return os.Setenv("DOCKER_HOST", "unix://"+socket)
}

func harden(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodPost && strings.HasSuffix(r.URL.Path, "/containers/create") {
			body, err := hardenCreateBody(r.Body)
			if err != nil {
				http.Error(w, err.Error(), http.StatusBadRequest)
				return
			}
			r.Body = io.NopCloser(bytes.NewReader(body))
			r.ContentLength = int64(len(body))
			r.Header.Set("Content-Length", strconv.Itoa(len(body)))
		}
		next.ServeHTTP(w, r)
	})
}

func hardenCreateBody(body io.ReadCloser) ([]byte, error) {
	defer body.Close()
	var cfg map[string]interface{}
	if err := json.NewDecoder(body).Decode(&cfg); err != nil {
		return nil, fmt.Errorf("invalid container create body: %w", err)
	}

	hostConfig, _ := cfg["HostConfig"].(map[string]interface{})
	if hostConfig == nil {
		hostConfig = make(map[string]interface{})
	}
	hostConfig["Privileged"] = false
	hostConfig["CapAdd"] = nil
	hostConfig["CapDrop"] = []string{"ALL"}

	securityOpt := []interface{}{"no-new-privileges"}
	if existing, ok := hostConfig["SecurityOpt"].([]interface{}); ok {
		for _, opt := range existing {
			if s, _ := opt.(string); !strings.HasPrefix(s, "no-new-privileges") {
				securityOpt = append(securityOpt, opt)
			}
		}
	}
	hostConfig["SecurityOpt"] = securityOpt

	cfg["HostConfig"] = hostConfig
	return json.Marshal(cfg)
}
