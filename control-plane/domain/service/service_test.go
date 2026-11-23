package service

import (
	"testing"

	"github.com/google/uuid"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/app"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/resource"
)

func TestCreateService(t *testing.T) {
	newApp := app.App{
		ID:       "app-123",
		CodePath: "job/main.go",
		Command:  "go run main.go",
	}

	resources := resource.Resource{
		CPU:    2,
		Memory: 512,
	}

	name := "test-service"

	newService := CreateService(newApp, resources, name)

	// Check generated service ID
	if _, err := uuid.Parse(newService.ID); err != nil {
		t.Errorf("expected valid service UUID, got %q", newService.ID)
	}

	// Check generated system ID
	if _, err := uuid.Parse(newService.SystemID); err != nil {
		t.Errorf("expected valid system UUID, got %q", newService.SystemID)
	}

	// Check name
	if newService.Name != name {
		t.Errorf("expected name %q, got %q", name, newService.Name)
	}

	// Check app
	if newService.App != newApp {
		t.Errorf("expected app %+v, got %+v", newApp, newService.App)
	}

	// Check desired replicas
	if newService.DesiredReplicas != 1 {
		t.Errorf("expected DesiredReplicas 1, got %d", newService.DesiredReplicas)
	}

	// Check resources
	if newService.Resources != resources {
		t.Errorf("expected resources %+v, got %+v", resources, newService.Resources)
	}

	// Check network initialization
	if newService.Network == nil {
		t.Fatal("expected initialized Network slice, got nil")
	}

	if len(newService.Network) != 0 {
		t.Errorf("expected empty Network slice, got %d rules", len(newService.Network))
	}
}

func TestCreateService_GeneratesUniqueIDs(t *testing.T) {
	newApp := app.App{}
	resources := resource.Resource{}

	firstService := CreateService(newApp, resources, "service-1")
	secondService := CreateService(newApp, resources, "service-2")

	if firstService.ID == secondService.ID {
		t.Error("expected unique service IDs")
	}

	if firstService.SystemID == secondService.SystemID {
		t.Error("expected unique system IDs")
	}
}
