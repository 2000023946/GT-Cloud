package data_plane

import (
	"errors"
	"io"
	"testing"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/globals"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/resource"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/application"
)

// Mock HTTP Adapter
type mockHttpAdapter struct {
	response    application.HttpResponse
	err         error
	url         string
	called      bool
	contentType string
	body        []byte
	postErr     error
}

func (m *mockHttpAdapter) Get(url string) (application.HttpResponse, error) {
	m.called = true
	m.url = url
	return m.response, m.err
}

func (m *mockHttpAdapter) Post(url string, contentType string, body io.Reader) (application.HttpResponse, error) {
	m.called = true
	m.url = url
	m.contentType = contentType

	if body != nil {
		m.body, _ = io.ReadAll(body)
	}

	return m.response, m.postErr
}

// Mock JSON Adapter
type mockJsonAdapter struct {
	payload    []byte
	err        error
	called     bool
	marshalErr error
	resource   resource.Resource
}

func (m *mockJsonAdapter) Marshal(content any) ([]byte, error) {
	return m.payload, m.marshalErr
}

func (m *mockJsonAdapter) Unmarshal(data []byte, v any) error {
	m.called = true

	if m.err != nil {
		return m.err
	}

	result, ok := v.(*resource.Resource)
	if ok {
		*result = m.resource
	}

	return nil
}

var _ application.IHttpAdapter = (*mockHttpAdapter)(nil)
var _ application.IJsonAdapter = (*mockJsonAdapter)(nil)

func TestDataPlane_StartJob(t *testing.T) {
	t.Run("successful start job", func(t *testing.T) {
		jsonAdapter := &mockJsonAdapter{
			payload: []byte(`{"id":"service-1"}`),
		}

		httpAdapter := &mockHttpAdapter{}

		dataPlane := &DataPlane{
			jsonAdapter: jsonAdapter,
			httpAdapter: httpAdapter,
		}

		newService := service.Service{
			ID: "service-1",
		}

		err := dataPlane.StartJob(newService)

		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}

		if !httpAdapter.called {
			t.Fatal("expected HTTP Post to be called")
		}

		if httpAdapter.contentType != "application/json" {
			t.Errorf("expected application/json, got %s", httpAdapter.contentType)
		}

		if string(httpAdapter.body) != string(jsonAdapter.payload) {
			t.Errorf("expected body %s, got %s", jsonAdapter.payload, httpAdapter.body)
		}
	})

	t.Run("JSON marshal error", func(t *testing.T) {
		expectedErr := errors.New("marshal failed")

		jsonAdapter := &mockJsonAdapter{
			marshalErr: expectedErr,
		}

		httpAdapter := &mockHttpAdapter{}

		dataPlane := &DataPlane{
			jsonAdapter: jsonAdapter,
			httpAdapter: httpAdapter,
		}

		err := dataPlane.StartJob(service.Service{})

		if err != expectedErr {
			t.Errorf("expected error %v, got %v", expectedErr, err)
		}

		if httpAdapter.called {
			t.Error("HTTP Post should not be called after marshal failure")
		}
	})

	t.Run("HTTP post error", func(t *testing.T) {
		expectedErr := errors.New("HTTP request failed")

		jsonAdapter := &mockJsonAdapter{
			payload: []byte(`{"id":"service-1"}`),
		}

		httpAdapter := &mockHttpAdapter{
			postErr: expectedErr,
		}

		dataPlane := &DataPlane{
			jsonAdapter: jsonAdapter,
			httpAdapter: httpAdapter,
		}

		err := dataPlane.StartJob(service.Service{})

		if err != expectedErr {
			t.Errorf("expected error %v, got %v", expectedErr, err)
		}

		if !httpAdapter.called {
			t.Error("expected HTTP Post to be called")
		}
	})
}

func TestDataPlane_GetJobLogs(t *testing.T) {
	tests := []struct {
		name           string
		httpResponse   application.HttpResponse
		httpError      error
		jsonError      error
		expectError    bool
		expectHTTPCall bool
	}{
		{
			name: "successful GET request",
			httpResponse: application.HttpResponse{
				StatusCode: 200,
				Body:       []byte(`{"stdout":"Job started","stderr":""}`),
			},
			expectHTTPCall: true,
		},
		{
			name:           "HTTP request failure",
			httpError:      errors.New("connection refused"),
			expectError:    true,
			expectHTTPCall: true,
		},
		{
			name: "JSON unmarshalling failure",
			httpResponse: application.HttpResponse{
				StatusCode: 200,
				Body:       []byte(`invalid json`),
			},
			jsonError:      errors.New("invalid JSON"),
			expectError:    true,
			expectHTTPCall: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			httpMock := &mockHttpAdapter{
				response: tt.httpResponse,
				err:      tt.httpError,
			}

			jsonMock := &mockJsonAdapter{
				err: tt.jsonError,
			}

			dp := &DataPlane{
				httpAdapter: httpMock,
				jsonAdapter: jsonMock,
			}

			_, err := dp.GetJobLogs("job-123")

			if (err != nil) != tt.expectError {
				t.Errorf("expected error %v, got %v", tt.expectError, err)
			}

			if httpMock.called != tt.expectHTTPCall {
				t.Errorf("expected HTTP call %v, got %v", tt.expectHTTPCall, httpMock.called)
			}

			if tt.expectHTTPCall && httpMock.url == "" {
				t.Error("expected a non-empty URL")
			}
		})
	}
}

func TestDataPlane_GetJobResources(t *testing.T) {
	t.Run("successful resource retrieval", func(t *testing.T) {
		jsonAdapter := &mockJsonAdapter{
			resource: resource.Resource{
				CPU:    2.5,
				Memory: 512,
			},
		}

		httpAdapter := &mockHttpAdapter{
			response: application.HttpResponse{
				StatusCode: 200,
				Body:       []byte(`{"CPU":2.5,"Memory":512}`),
			},
		}

		dataPlane := &DataPlane{
			jsonAdapter: jsonAdapter,
			httpAdapter: httpAdapter,
		}

		result, err := dataPlane.GetJobResources("job-123")

		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}

		if result.CPU != 2.5 {
			t.Errorf("expected CPU 2.5, got %v", result.CPU)
		}

		if result.Memory != 512 {
			t.Errorf("expected Memory 512, got %v", result.Memory)
		}

		if !httpAdapter.called {
			t.Error("expected HTTP GET to be called")
		}

		if !jsonAdapter.called {
			t.Error("expected JSON Unmarshal to be called")
		}

		expectedURL := globals.DataPlaneUrl + "/job_resources/job-123"

		if httpAdapter.url != expectedURL {
			t.Errorf("expected URL %q, got %q", expectedURL, httpAdapter.url)
		}
	})

	t.Run("HTTP GET returns error", func(t *testing.T) {
		jsonAdapter := &mockJsonAdapter{}

		httpAdapter := &mockHttpAdapter{
			err: errors.New("connection failed"),
		}

		dataPlane := &DataPlane{
			jsonAdapter: jsonAdapter,
			httpAdapter: httpAdapter,
		}

		result, err := dataPlane.GetJobResources("job-123")

		if err == nil {
			t.Fatal("expected error when HTTP GET fails")
		}

		if result != (resource.Resource{}) {
			t.Errorf("expected empty resource, got %+v", result)
		}

		if jsonAdapter.called {
			t.Error("JSON Unmarshal should not be called when HTTP GET fails")
		}
	})

	t.Run("JSON unmarshal returns error", func(t *testing.T) {
		jsonAdapter := &mockJsonAdapter{
			err: errors.New("invalid JSON"),
		}

		httpAdapter := &mockHttpAdapter{
			response: application.HttpResponse{
				StatusCode: 200,
				Body:       []byte("invalid JSON"),
			},
		}

		dataPlane := &DataPlane{
			jsonAdapter: jsonAdapter,
			httpAdapter: httpAdapter,
		}

		result, err := dataPlane.GetJobResources("job-123")

		if err == nil {
			t.Fatal("expected error when JSON unmarshalling fails")
		}

		if result != (resource.Resource{}) {
			t.Errorf("expected empty resource, got %+v", result)
		}

		if !jsonAdapter.called {
			t.Error("expected JSON Unmarshal to be called")
		}
	})
}

func TestDataPlane_DeleteJob(t *testing.T) {
	t.Run("successful job deletion", func(t *testing.T) {
		httpAdapter := &mockHttpAdapter{}

		dataPlane := &DataPlane{
			httpAdapter: httpAdapter,
		}

		err := dataPlane.DeleteJob("job-123")

		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}

		if !httpAdapter.called {
			t.Fatal("expected HTTP GET to be called")
		}

		expectedURL := globals.DataPlaneUrl + "/delete_job/job-123"

		if httpAdapter.url != expectedURL {
			t.Errorf("expected URL %q, got %q", expectedURL, httpAdapter.url)
		}
	})

	t.Run("HTTP GET returns error", func(t *testing.T) {
		httpAdapter := &mockHttpAdapter{
			err: errors.New("connection failed"),
		}

		dataPlane := &DataPlane{
			httpAdapter: httpAdapter,
		}

		err := dataPlane.DeleteJob("job-123")

		if err == nil {
			t.Fatal("expected error when HTTP GET fails")
		}

		if !httpAdapter.called {
			t.Error("expected HTTP GET to be called")
		}

		expectedURL := globals.DataPlaneUrl + "/delete_job/job-123"

		if httpAdapter.url != expectedURL {
			t.Errorf("expected URL %q, got %q", expectedURL, httpAdapter.url)
		}
	})
}
