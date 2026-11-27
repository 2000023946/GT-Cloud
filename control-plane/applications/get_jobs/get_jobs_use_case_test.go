package get_jobs

import (
	"errors"
	"testing"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
)

// Mock repository
type MockServiceRepository struct {
	services []service.Service
	err      error
}

func (m *MockServiceRepository) Create(s service.Service) error {
	return nil
}

func (m *MockServiceRepository) Get(id string) (service.Service, error) {
	return service.Service{}, nil
}

func (m *MockServiceRepository) GetAll() ([]service.Service, error) {
	return m.services, m.err
}

func (m *MockServiceRepository) Delete(id string) error {
	return nil
}

func TestGetJobsUseCaseExecuteSuccess(t *testing.T) {
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

	mockRepo := &MockServiceRepository{
		services: services,
		err:      nil,
	}

	useCase := GetJobsUseCase{
		serviceRepo: mockRepo,
	}

	response := useCase.execute()

	if response.IsError {
		t.Errorf("expected IsError to be false, got true")
	}

	if response.Message != "" {
		t.Errorf("expected empty Message, got %q", response.Message)
	}

	if len(response.Jobs) != len(services) {
		t.Fatalf("expected %d jobs, got %d", len(services), len(response.Jobs))
	}

	for i, service := range services {
		if response.Jobs[i].JobId != service.ID {
			t.Errorf("job %d: expected JobId %q, got %q",
				i, service.ID, response.Jobs[i].JobId)
		}

		if response.Jobs[i].Name != service.Name {
			t.Errorf("job %d: expected Name %q, got %q",
				i, service.Name, response.Jobs[i].Name)
		}
	}
}

func TestGetJobsUseCaseExecuteRepositoryError(t *testing.T) {
	repoError := errors.New("database connection failed")

	mockRepo := &MockServiceRepository{
		services: nil,
		err:      repoError,
	}

	useCase := GetJobsUseCase{
		serviceRepo: mockRepo,
	}

	response := useCase.execute()

	if !response.IsError {
		t.Errorf("expected IsError to be true, got false")
	}

	if response.Message != repoError.Error() {
		t.Errorf("expected Message %q, got %q",
			repoError.Error(), response.Message)
	}

	if len(response.Jobs) != 0 {
		t.Errorf("expected 0 jobs, got %d", len(response.Jobs))
	}
}

func TestGetJobsUseCaseExecuteEmptyServices(t *testing.T) {
	mockRepo := &MockServiceRepository{
		services: []service.Service{},
		err:      nil,
	}

	useCase := GetJobsUseCase{
		serviceRepo: mockRepo,
	}

	response := useCase.execute()

	if response.IsError {
		t.Errorf("expected IsError to be false, got true")
	}

	if len(response.Jobs) != 0 {
		t.Errorf("expected 0 jobs, got %d", len(response.Jobs))
	}
}
