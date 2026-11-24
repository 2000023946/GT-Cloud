package app

import (
	"testing"

	"github.com/google/uuid"
)

func TestCreateApp(t *testing.T) {
	codepath := "job/test/main.go"
	command := "go run main.go"

	newApp := CreateApp(codepath, command)

	if newApp == nil {
		t.Fatal("expected app, got nil")
	}

	if newApp.CodePath != codepath {
		t.Errorf("expected CodePath %q, got %q", codepath, newApp.CodePath)
	}

	if newApp.Command != command {
		t.Errorf("expected Command %q, got %q", command, newApp.Command)
	}

	if _, err := uuid.Parse(newApp.ID); err != nil {
		t.Errorf("expected valid UUID, got %q", newApp.ID)
	}
}

func TestCreateApp_GeneratesUniqueIDs(t *testing.T) {
	firstApp := CreateApp("path1", "command1")
	secondApp := CreateApp("path2", "command2")

	if firstApp.ID == secondApp.ID {
		t.Error("expected different IDs, got identical IDs")
	}
}
