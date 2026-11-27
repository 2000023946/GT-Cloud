package stop_job

import (
	"errors"
	"testing"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/job_logs"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/resource"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
)

type MockServiceRepository struct {
	DeleteFunc  func(id string) error
	DeleteCalls int
	DeletedID   string
}

func (m *MockServiceRepository) Create(s service.Service) error {
	return nil
}

func (m *MockServiceRepository) Get(id string) (service.Service, error) {
	return service.Service{}, nil
}

func (m *MockServiceRepository) GetAll() ([]service.Service, error) {
	return nil, nil
}

func (m *MockServiceRepository) Delete(id string) error {
	m.DeleteCalls++
	m.DeletedID = id
	if m.DeleteFunc != nil {
		return m.DeleteFunc(id)
	}
	return nil
}

type MockDataPlane struct {
	DeleteJobFunc  func(jobId string) error
	DeleteJobCalls int
	DeletedJobID   string
}

func (m *MockDataPlane) StartJob(s service.Service) error {
	return nil
}

func (m *MockDataPlane) GetJobLogs(jobId string) (job_logs.JobLogs, error) {
	return job_logs.JobLogs{}, nil
}

func (m *MockDataPlane) GetJobResources(jobId string) (resource.Resource, error) {
	return resource.Resource{}, nil
}

func (m *MockDataPlane) DeleteJob(jobId string) error {
	m.DeleteJobCalls++
	m.DeletedJobID = jobId
	if m.DeleteJobFunc != nil {
		return m.DeleteJobFunc(jobId)
	}
	return nil
}

func TestStopJobUseCaseSuccess(t *testing.T) {
	repo := &MockServiceRepository{}
	dataPlane := &MockDataPlane{}

	useCase := StopJobUseCase{
		serviceRepo: repo,
		dataPlane:   dataPlane,
	}

	response := useCase.execute(StopJobRequest{JobID: "job-123"})

	if !response.IsSuccess {
		t.Errorf("expected IsSuccess to be true")
	}

	if response.IsError {
		t.Errorf("expected IsError to be false")
	}

	if dataPlane.DeleteJobCalls != 1 {
		t.Errorf("expected 1 data plane DeleteJob call, got %d", dataPlane.DeleteJobCalls)
	}

	if repo.DeleteCalls != 1 {
		t.Errorf("expected 1 repository Delete call, got %d", repo.DeleteCalls)
	}

	if dataPlane.DeletedJobID != "job-123" {
		t.Errorf("expected data plane job ID job-123, got %q", dataPlane.DeletedJobID)
	}

	if repo.DeletedID != "job-123" {
		t.Errorf("expected repository job ID job-123, got %q", repo.DeletedID)
	}
}

func TestStopJobUseCaseDataPlaneError(t *testing.T) {
	repo := &MockServiceRepository{}
	dataPlane := &MockDataPlane{
		DeleteJobFunc: func(jobId string) error {
			return errors.New("data plane deletion failed")
		},
	}

	useCase := StopJobUseCase{
		serviceRepo: repo,
		dataPlane:   dataPlane,
	}

	response := useCase.execute(StopJobRequest{JobID: "job-123"})

	if response.IsSuccess {
		t.Errorf("expected IsSuccess to be false")
	}

	if !response.IsError {
		t.Errorf("expected IsError to be true")
	}

	if repo.DeleteCalls != 0 {
		t.Errorf("expected repository Delete not to be called, got %d calls", repo.DeleteCalls)
	}

	if dataPlane.DeleteJobCalls != 1 {
		t.Errorf("expected 1 data plane DeleteJob call, got %d", dataPlane.DeleteJobCalls)
	}
}

func TestStopJobUseCaseRepositoryError(t *testing.T) {
	repo := &MockServiceRepository{
		DeleteFunc: func(id string) error {
			return errors.New("repository deletion failed")
		},
	}
	dataPlane := &MockDataPlane{}

	useCase := StopJobUseCase{
		serviceRepo: repo,
		dataPlane:   dataPlane,
	}

	response := useCase.execute(StopJobRequest{JobID: "job-123"})

	if response.IsSuccess {
		t.Errorf("expected IsSuccess to be false")
	}

	if !response.IsError {
		t.Errorf("expected IsError to be true")
	}

	if dataPlane.DeleteJobCalls != 1 {
		t.Errorf("expected 1 data plane DeleteJob call, got %d", dataPlane.DeleteJobCalls)
	}

	if repo.DeleteCalls != 1 {
		t.Errorf("expected 1 repository Delete call, got %d", repo.DeleteCalls)
	}
}
