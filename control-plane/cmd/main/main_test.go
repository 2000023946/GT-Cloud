package main

import (
	"errors"
	"net/http"
	"testing"
)

func TestStartServer(t *testing.T) {
	expectedErr := errors.New("server error")

	original := listenAndServe
	defer func() {
		listenAndServe = original
	}()

	listenAndServe = func(addr string, handler http.Handler) error {
		if addr != ":8080" {
			t.Errorf("expected address :8080, got %s", addr)
		}

		if handler != nil {
			t.Errorf("expected nil handler")
		}

		return expectedErr
	}

	err := startServer(":8080")

	if !errors.Is(err, expectedErr) {
		t.Fatalf("expected %v, got %v", expectedErr, err)
	}
}

func TestMain(t *testing.T) {
	expectedErr := errors.New("server error")

	original := listenAndServe
	defer func() {
		listenAndServe = original
	}()

	listenAndServe = func(addr string, handler http.Handler) error {
		if addr != ":8080" {
			t.Errorf("expected address :8080, got %s", addr)
		}

		return expectedErr
	}

	main()
}
