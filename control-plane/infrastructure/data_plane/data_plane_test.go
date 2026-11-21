package data_plane

import (
	"errors"
	"io"
	"net/http"
	"testing"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
)

// Mock JSON adapter
type mockJsonAdapter struct {
	marshalErr error
	payload    []byte
}

func (m *mockJsonAdapter) Marshal(content any) ([]byte, error) {
	return m.payload, m.marshalErr
}

// Mock HTTP adapter
type mockHttpAdapter struct {
	postErr     error
	called      bool
	url         string
	contentType string
	body        []byte
}

func (m *mockHttpAdapter) Post(url string, contentType string, body io.Reader) (any, error) {
	m.called = true
	m.url = url
	m.contentType = contentType

	m.body, _ = io.ReadAll(body)

	return &http.Response{StatusCode: http.StatusOK}, m.postErr
}

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
