package get_job_logs

import (
	"errors"
	"testing"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/job_logs"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/application"
)

type MockDataPlane struct {
	logs        job_logs.JobLogs
	err         error
	called      bool
	calledJobId string
}

func (m *MockDataPlane) StartJob(service service.Service) error {
	return nil
}

func (m *MockDataPlane) GetJobLogs(jobId string) (job_logs.JobLogs, error) {
	m.called = true
	m.calledJobId = jobId
	return m.logs, m.err
}

var _ application.IDataPlane = (*MockDataPlane)(nil)

func TestGetJobLogsUseCaseExecute(t *testing.T) {
	tests := []struct {
		name           string
		request        GetJobLogsRequest
		mockLogs       job_logs.JobLogs
		mockError      error
		expectedError  bool
		expectedMsg    string
		expectedStdout string
		expectedStderr string
		expectCall     bool
	}{
		{
			name:    "successful execution",
			request: GetJobLogsRequest{JobId: "job-123"},
			mockLogs: job_logs.JobLogs{
				Stdout: "Job started",
				Stderr: "",
			},
			expectCall:     true,
			expectedStdout: "Job started",
		},
		{
			name:          "invalid request",
			request:       GetJobLogsRequest{JobId: ""},
			expectCall:    false,
			expectedError: true,
		},
		{
			name:          "data plane error",
			request:       GetJobLogsRequest{JobId: "job-456"},
			mockError:     errors.New("failed to retrieve logs"),
			expectCall:    true,
			expectedError: true,
		},
		{
			name:       "empty logs",
			request:    GetJobLogsRequest{JobId: "job-789"},
			mockLogs:   job_logs.JobLogs{},
			expectCall: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			mock := &MockDataPlane{
				logs: tt.mockLogs,
				err:  tt.mockError,
			}

			useCase := GetJobLogsUseCase{
				DataPlane: mock,
			}

			response := useCase.Execute(tt.request)

			if response.isError != tt.expectedError {
				t.Errorf("expected isError %v, got %v", tt.expectedError, response.isError)
			}

			if response.stdout != tt.expectedStdout {
				t.Errorf("expected stdout %q, got %q", tt.expectedStdout, response.stdout)
			}

			if mock.called != tt.expectCall {
				t.Errorf("expected DataPlane called %v, got %v", tt.expectCall, mock.called)
			}

			if tt.expectCall && mock.calledJobId != tt.request.JobId {
				t.Errorf("expected JobId %q, got %q", tt.request.JobId, mock.calledJobId)
			}
		})
	}
}
