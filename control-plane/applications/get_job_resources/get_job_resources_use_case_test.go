package get_job_resources

import (
	"errors"
	"testing"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/job_logs"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/resource"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/application"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/repository"
)

// Mock service repository
type MockServiceRepository struct {
	Service   service.Service
	GetError  error
	GetCalled bool
}

func (m *MockServiceRepository) Create(s service.Service) error {
	return nil
}

func (m *MockServiceRepository) Get(id string) (service.Service, error) {
	m.GetCalled = true
	return m.Service, m.GetError
}

func (m *MockServiceRepository) GetAll() ([]service.Service, error) {
	return nil, nil
}

// Mock data plane
type MockDataPlane struct {
	Resource       resource.Resource
	GetError       error
	GetCalled      bool
	RequestedJobID string
}

func (m *MockDataPlane) StartJob(s service.Service) error {
	return nil
}

func (m *MockDataPlane) GetJobLogs(jobId string) (job_logs.JobLogs, error) {
	return job_logs.JobLogs{}, nil
}

func (m *MockDataPlane) GetJobResources(jobId string) (resource.Resource, error) {
	m.GetCalled = true
	m.RequestedJobID = jobId
	return m.Resource, m.GetError
}

var _ repository.IServiceRepository = (*MockServiceRepository)(nil)
var _ application.IDataPlane = (*MockDataPlane)(nil)

func TestGetJobResourcesUseCase_Create(t *testing.T) {
	t.Run("job does not exist", func(t *testing.T) {
		repository := &MockServiceRepository{
			GetError: errors.New("job not found"),
		}

		dataPlane := &MockDataPlane{}

		useCase := GetJobResourcesUseCase{
			service_repository: repository,
			data_plane:         dataPlane,
		}

		response := useCase.Create(GetJobResourcesRequest{
			JobId: "job-123",
		})

		if !response.IsError {
			t.Fatal("expected error response")
		}

		if response.Message == "" {
			t.Fatal("expected error message")
		}

		if dataPlane.GetCalled {
			t.Error("data plane should not be called when job does not exist")
		}
	})

	t.Run("data plane returns error", func(t *testing.T) {
		repository := &MockServiceRepository{
			Service: service.Service{ID: "job-123"},
		}

		dataPlane := &MockDataPlane{
			GetError: errors.New("failed to retrieve resources"),
		}

		useCase := GetJobResourcesUseCase{
			service_repository: repository,
			data_plane:         dataPlane,
		}

		response := useCase.Create(GetJobResourcesRequest{
			JobId: "job-123",
		})

		if !response.IsError {
			t.Fatal("expected error response")
		}

		if response.Message == "" {
			t.Fatal("expected error message")
		}

		if !dataPlane.GetCalled {
			t.Error("expected data plane to be called")
		}
	})

	t.Run("successful resource retrieval", func(t *testing.T) {
		repository := &MockServiceRepository{
			Service: service.Service{ID: "job-123"},
		}

		dataPlane := &MockDataPlane{
			Resource: resource.Resource{
				CPU:    2.5,
				Memory: 512,
			},
		}

		useCase := GetJobResourcesUseCase{
			service_repository: repository,
			data_plane:         dataPlane,
		}

		response := useCase.Create(GetJobResourcesRequest{
			JobId: "job-123",
		})

		if response.IsError {
			t.Fatalf("expected success response, got error: %s", response.Message)
		}

		if response.Message != "" {
			t.Errorf("expected empty message, got %q", response.Message)
		}

		if response.CpuUsed != 2.5 {
			t.Errorf("expected CpuUsed 2.5, got %v", response.CpuUsed)
		}

		if response.MemoryUsed != 512 {
			t.Errorf("expected MemoryUsed 512, got %v", response.MemoryUsed)
		}

		if !dataPlane.GetCalled {
			t.Error("expected data plane to be called")
		}

		if dataPlane.RequestedJobID != "job-123" {
			t.Errorf("expected job ID job-123, got %s", dataPlane.RequestedJobID)
		}
	})
}
