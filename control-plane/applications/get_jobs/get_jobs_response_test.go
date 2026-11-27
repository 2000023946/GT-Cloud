package get_jobs

import (
	"testing"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
)

func TestGetJobsResponseSuccess(t *testing.T) {
	services := []service.Service{
		{
			ID:   "service-1",
			Name: "web-server",
		},
		{
			ID:   "service-2",
			Name: "database",
		},
	}

	response := GetJobsResponseSuccess(services)

	if response.IsError {
		t.Errorf("expected IsError to be false, got true")
	}

	if response.Message != "" {
		t.Errorf("expected empty Message, got %q", response.Message)
	}

	expectedDtos := CreateJobDtos(services)

	if len(response.Jobs) != len(expectedDtos) {
		t.Fatalf(
			"expected %d jobs, got %d",
			len(expectedDtos),
			len(response.Jobs),
		)
	}

	for i, expected := range expectedDtos {
		if response.Jobs[i].JobId != expected.JobId {
			t.Errorf(
				"job %d: expected JobId %q, got %q",
				i,
				expected.JobId,
				response.Jobs[i].JobId,
			)
		}

		if response.Jobs[i].Name != expected.Name {
			t.Errorf(
				"job %d: expected Name %q, got %q",
				i,
				expected.Name,
				response.Jobs[i].Name,
			)
		}
	}
}

func TestGetJobsResponseSuccessEmptyServices(t *testing.T) {
	response := GetJobsResponseSuccess([]service.Service{})

	if response.IsError {
		t.Errorf("expected IsError to be false, got true")
	}

	if response.Message != "" {
		t.Errorf("expected empty Message, got %q", response.Message)
	}

	if len(response.Jobs) != 0 {
		t.Errorf("expected 0 jobs, got %d", len(response.Jobs))
	}
}

func TestGetJobsResponseError(t *testing.T) {
	message := "failed to retrieve jobs"

	response := GetJobsResponseError(message)

	if !response.IsError {
		t.Errorf("expected IsError to be true, got false")
	}

	if response.Message != message {
		t.Errorf("expected Message %q, got %q", message, response.Message)
	}

	if len(response.Jobs) != 0 {
		t.Errorf("expected 0 jobs, got %d", len(response.Jobs))
	}
}
