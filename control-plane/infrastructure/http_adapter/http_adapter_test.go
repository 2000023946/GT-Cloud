package http_adapter

import (
	"io"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

func TestHttpAdapter_Post(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			t.Errorf("expected POST, got %s", r.Method)
		}

		if r.Header.Get("Content-Type") != "application/json" {
			t.Errorf("expected application/json, got %s", r.Header.Get("Content-Type"))
		}

		body, err := io.ReadAll(r.Body)
		if err != nil {
			t.Errorf("failed to read request body: %v", err)
		}

		if string(body) != `{"name":"test"}` {
			t.Errorf("unexpected body: %s", body)
		}

		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	adapter := &HttpAdapter{}

	response, err := adapter.Post(
		server.URL,
		"application/json",
		strings.NewReader(`{"name":"test"}`),
	)

	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if response == nil {
		t.Fatal("expected response, got nil")
	}

	httpResponse, ok := response.(*http.Response)
	if !ok {
		t.Fatalf("expected *http.Response, got %T", response)
	}

	defer httpResponse.Body.Close()

	if httpResponse.StatusCode != http.StatusOK {
		t.Errorf("expected status 200, got %d", httpResponse.StatusCode)
	}
}
