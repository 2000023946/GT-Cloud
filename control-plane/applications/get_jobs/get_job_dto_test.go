package get_jobs

import (
	"testing"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
)

func TestCreateJobDtos(t *testing.T) {
	services := []service.Service{
		{
			ID:   "service-1",
			Name: "web-server",
		},
		{
			ID:   "service-2",
			Name: "database",
		},
		{
			ID:   "service-3",
			Name: "cache",
		},
	}

	dtos := CreateJobDtos(services)

	if len(dtos) != len(services) {
		t.Fatalf("expected %d DTOs, got %d", len(services), len(dtos))
	}

	for i, service := range services {
		if dtos[i].JobId != service.ID {
			t.Errorf(
				"DTO %d: expected JobId %q, got %q",
				i,
				service.ID,
				dtos[i].JobId,
			)
		}

		if dtos[i].Name != service.Name {
			t.Errorf(
				"DTO %d: expected Name %q, got %q",
				i,
				service.Name,
				dtos[i].Name,
			)
		}
	}
}

func TestCreateJobDtosEmptyServices(t *testing.T) {
	services := []service.Service{}

	dtos := CreateJobDtos(services)

	if len(dtos) != 0 {
		t.Errorf("expected 0 DTOs, got %d", len(dtos))
	}
}

func TestCreateJobDtosNilServices(t *testing.T) {
	var services []service.Service

	dtos := CreateJobDtos(services)

	if len(dtos) != 0 {
		t.Errorf("expected 0 DTOs, got %d", len(dtos))
	}
}
