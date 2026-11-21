package start_job

import (
	"errors"
	"mime/multipart"
	"testing"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/application"
)

// Mock file
type mockCodeFile struct{}

func (m *mockCodeFile) Filename() string {
	return "main.go"
}

func (m *mockCodeFile) Open() (multipart.File, error) {
	return nil, nil
}

func (m *mockCodeFile) Size() int {
	return 100
}

// Mock code repository
type mockCodeRepository struct {
	err    error
	called bool
}

func (m *mockCodeRepository) Save(codeFile application.IFile) (string, error) {
	m.called = true
	if m.err != nil {
		return "", m.err
	}
	return "job/test-id/main.go", nil
}

// Mock service repository
type mockServiceRepository struct {
	err    error
	called bool
}

func (m *mockServiceRepository) Create(newService service.Service) error {
	m.called = true
	return m.err
}

// Mock data plane
type mockDataPlane struct {
	err    error
	called bool
}

func (m *mockDataPlane) StartJob(newService service.Service) error {
	m.called = true
	return m.err
}

// Test use case
func TestStartJobUseCase_Execute(t *testing.T) {

	t.Run("invalid request", func(t *testing.T) {
		codeRepo := &mockCodeRepository{}
		serviceRepo := &mockServiceRepository{}
		dataPlane := &mockDataPlane{}

		useCase := StartJobUseCase{
			codeRepo:    codeRepo,
			serviceRepo: serviceRepo,
			dataPlane:   dataPlane,
		}

		request := StartJobRequest{}

		response, err := useCase.Execute(request)

		if err != nil {
			t.Fatalf("expected nil error, got %v", err)
		}

		if !response.IsError {
			t.Error("expected error response")
		}

		if codeRepo.called || serviceRepo.called || dataPlane.called {
			t.Error("dependencies should not be called for invalid request")
		}
	})

	t.Run("invalid resource", func(t *testing.T) {
		codeRepo := &mockCodeRepository{}
		serviceRepo := &mockServiceRepository{}
		dataPlane := &mockDataPlane{}

		useCase := StartJobUseCase{
			codeRepo:    codeRepo,
			serviceRepo: serviceRepo,
			dataPlane:   dataPlane,
		}

		request := StartJobRequest{
			Name:           "test-job",
			StartCommand:   "go run main.go",
			CodeFile:       &mockCodeFile{},
			CpuResource:    0,
			MemoryResource: 512,
		}

		response, err := useCase.Execute(request)

		if err != nil {
			t.Fatalf("expected nil error, got %v", err)
		}

		if !response.IsError {
			t.Error("expected error response")
		}

		if codeRepo.called || serviceRepo.called || dataPlane.called {
			t.Error("dependencies should not be called for invalid resource")
		}
	})

	t.Run("code repository error", func(t *testing.T) {
		expectedErr := errors.New("code save failed")

		codeRepo := &mockCodeRepository{err: expectedErr}
		serviceRepo := &mockServiceRepository{}
		dataPlane := &mockDataPlane{}

		useCase := StartJobUseCase{
			codeRepo:    codeRepo,
			serviceRepo: serviceRepo,
			dataPlane:   dataPlane,
		}

		request := validStartJobRequest()

		response, err := useCase.Execute(request)

		if err != nil {
			t.Fatalf("expected nil error, got %v", err)
		}

		if !response.IsError {
			t.Error("expected error response")
		}

		if response.Message != expectedErr.Error() {
			t.Errorf("expected message %q, got %q", expectedErr.Error(), response.Message)
		}

		if !codeRepo.called {
			t.Error("expected code repository to be called")
		}

		if serviceRepo.called || dataPlane.called {
			t.Error("service repository and data plane should not be called")
		}
	})

	t.Run("service repository error", func(t *testing.T) {
		expectedErr := errors.New("service creation failed")

		codeRepo := &mockCodeRepository{}
		serviceRepo := &mockServiceRepository{err: expectedErr}
		dataPlane := &mockDataPlane{}

		useCase := StartJobUseCase{
			codeRepo:    codeRepo,
			serviceRepo: serviceRepo,
			dataPlane:   dataPlane,
		}

		request := validStartJobRequest()

		response, err := useCase.Execute(request)

		if err != nil {
			t.Fatalf("expected nil error, got %v", err)
		}

		if !response.IsError {
			t.Error("expected error response")
		}

		if response.Message != expectedErr.Error() {
			t.Errorf("expected message %q, got %q", expectedErr.Error(), response.Message)
		}

		if !codeRepo.called || !serviceRepo.called {
			t.Error("expected code repository and service repository to be called")
		}

		if dataPlane.called {
			t.Error("data plane should not be called")
		}
	})

	t.Run("data plane error", func(t *testing.T) {
		expectedErr := errors.New("data plane failed")

		codeRepo := &mockCodeRepository{}
		serviceRepo := &mockServiceRepository{}
		dataPlane := &mockDataPlane{err: expectedErr}

		useCase := StartJobUseCase{
			codeRepo:    codeRepo,
			serviceRepo: serviceRepo,
			dataPlane:   dataPlane,
		}

		request := validStartJobRequest()

		response, err := useCase.Execute(request)

		if err != nil {
			t.Fatalf("expected nil error, got %v", err)
		}

		if !response.IsError {
			t.Error("expected error response")
		}

		if response.Message != expectedErr.Error() {
			t.Errorf("expected message %q, got %q", expectedErr.Error(), response.Message)
		}

		if !codeRepo.called || !serviceRepo.called || !dataPlane.called {
			t.Error("expected all dependencies to be called")
		}
	})

	t.Run("successful execution", func(t *testing.T) {
		codeRepo := &mockCodeRepository{}
		serviceRepo := &mockServiceRepository{}
		dataPlane := &mockDataPlane{}

		useCase := StartJobUseCase{
			codeRepo:    codeRepo,
			serviceRepo: serviceRepo,
			dataPlane:   dataPlane,
		}

		request := validStartJobRequest()

		response, err := useCase.Execute(request)

		if err != nil {
			t.Fatalf("expected nil error, got %v", err)
		}

		if response.IsError {
			t.Errorf("expected successful response, got %s", response.Message)
		}

		if response.Message != "Job has successfully been started" {
			t.Errorf("unexpected response message: %s", response.Message)
		}

		if !codeRepo.called || !serviceRepo.called || !dataPlane.called {
			t.Error("expected all dependencies to be called")
		}
	})
}

func validStartJobRequest() StartJobRequest {
	return StartJobRequest{
		Name:           "test-job",
		StartCommand:   "go run main.go",
		CodeFile:       &mockCodeFile{},
		CpuResource:    1,
		MemoryResource: 1,
	}
}
