package http_adapter

import (
	"errors"
	"io"
	"net/http"
	"net/http/httptest"
	"os"
	"strings"
	"testing"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/application"
)

type MockDiskManager struct {
	readCalled bool
	readErr    error
}

func (m *MockDiskManager) ReadAll(r io.Reader) ([]byte, error) {
	m.readCalled = true

	if m.readErr != nil {
		return nil, m.readErr
	}

	return io.ReadAll(r)
}

func (m *MockDiskManager) WriteFile(name string, data []byte, perm os.FileMode) error {
	return nil
}

var _ application.IDiskManager = (*MockDiskManager)(nil)

func TestHttpAdapter_Post(t *testing.T) {
	tests := []struct {
		name           string
		handler        http.HandlerFunc
		contentType    string
		body           string
		readErr        error
		expectError    bool
		expectedStatus int
		expectedBody   string
		expectRead     bool
	}{
		{
			name:        "successful POST",
			contentType: "application/json",
			body:        `{"jobId":"job-123"}`,
			handler: func(w http.ResponseWriter, r *http.Request) {
				if r.Method != http.MethodPost {
					t.Errorf("expected POST, got %s", r.Method)
				}

				if r.Header.Get("Content-Type") != "application/json" {
					t.Errorf("unexpected content type: %s",
						r.Header.Get("Content-Type"))
				}

				requestBody, _ := io.ReadAll(r.Body)
				if string(requestBody) != `{"jobId":"job-123"}` {
					t.Errorf("unexpected request body: %s", requestBody)
				}

				w.WriteHeader(http.StatusOK)
				_, _ = w.Write([]byte("success"))
			},
			expectedStatus: http.StatusOK,
			expectedBody:   "success",
			expectRead:     true,
		},
		{
			name:        "HTTP server error",
			contentType: "application/json",
			body:        `{}`,
			handler: func(w http.ResponseWriter, r *http.Request) {
				w.WriteHeader(http.StatusInternalServerError)
				_, _ = w.Write([]byte("server error"))
			},
			expectError:    false,
			expectedStatus: http.StatusInternalServerError,
			expectedBody:   "server error",
			expectRead:     true,
		},
		{
			name:        "disk manager ReadAll error",
			contentType: "application/json",
			body:        `{}`,
			readErr:     errors.New("failed to read response"),
			handler: func(w http.ResponseWriter, r *http.Request) {
				w.WriteHeader(http.StatusOK)
				_, _ = w.Write([]byte("success"))
			},
			expectError: true,
			expectRead:  true,
		},
		{
			name:        "empty response body",
			contentType: "application/json",
			body:        `{}`,
			handler: func(w http.ResponseWriter, r *http.Request) {
				w.WriteHeader(http.StatusOK)
			},
			expectedStatus: http.StatusOK,
			expectedBody:   "",
			expectRead:     true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			server := httptest.NewServer(tt.handler)
			defer server.Close()

			diskManager := &MockDiskManager{
				readErr: tt.readErr,
			}

			adapter := &HttpAdapter{
				diskManager: diskManager,
			}

			response, err := adapter.Post(
				server.URL,
				tt.contentType,
				strings.NewReader(tt.body),
			)

			if tt.expectError && err == nil {
				t.Fatalf("expected error, got nil: test: [%s]", tt.name)

			}

			if !tt.expectError && err != nil {
				t.Fatalf("unexpected error: %v", err)
			}

			if tt.expectRead && !diskManager.readCalled {
				t.Fatal("expected disk manager ReadAll to be called")
			}

			if !tt.expectError {
				if response.StatusCode != tt.expectedStatus {
					t.Errorf("expected status %d, got %d",
						tt.expectedStatus, response.StatusCode)
				}

				if string(response.Body) != tt.expectedBody {
					t.Errorf("expected body %q, got %q",
						tt.expectedBody, string(response.Body))
				}
			}
		})
	}
}

func TestHttpAdapter_PostInvalidURL(t *testing.T) {
	diskManager := &MockDiskManager{}

	adapter := &HttpAdapter{
		diskManager: diskManager,
	}

	_, err := adapter.Post(
		"://invalid-url",
		"application/json",
		strings.NewReader("{}"),
	)

	if err == nil {
		t.Fatal("expected error for invalid URL")
	}

	if diskManager.readCalled {
		t.Fatal("ReadAll should not be called when the HTTP request fails")
	}
}

func TestHttpAdapter_Get(t *testing.T) {
	tests := []struct {
		name           string
		handler        http.HandlerFunc
		readErr        error
		expectError    bool
		expectedStatus int
		expectedBody   string
		expectRead     bool
	}{
		{
			name: "successful GET",
			handler: func(w http.ResponseWriter, r *http.Request) {
				if r.Method != http.MethodGet {
					t.Errorf("expected GET, got %s", r.Method)
				}

				w.WriteHeader(http.StatusOK)
				_, _ = w.Write([]byte(`{"stdout":"hello","stderr":""}`))
			},
			expectedStatus: http.StatusOK,
			expectedBody:   `{"stdout":"hello","stderr":""}`,
			expectRead:     true,
		},
		{
			name: "HTTP server error",
			handler: func(w http.ResponseWriter, r *http.Request) {
				w.WriteHeader(http.StatusInternalServerError)
				_, _ = w.Write([]byte("server error"))
			},
			expectError:    false,
			expectedStatus: http.StatusInternalServerError,
			expectedBody:   "server error",
			expectRead:     true,
		},
		{
			name:    "disk manager ReadAll error",
			readErr: errors.New("failed to read response"),
			handler: func(w http.ResponseWriter, r *http.Request) {
				w.WriteHeader(http.StatusOK)
				_, _ = w.Write([]byte("success"))
			},
			expectError: true,
			expectRead:  true,
		},
		{
			name: "empty response body",
			handler: func(w http.ResponseWriter, r *http.Request) {
				w.WriteHeader(http.StatusOK)
			},
			expectedStatus: http.StatusOK,
			expectedBody:   "",
			expectRead:     true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			server := httptest.NewServer(tt.handler)
			defer server.Close()

			diskManager := &MockDiskManager{
				readErr: tt.readErr,
			}

			adapter := &HttpAdapter{
				diskManager: diskManager,
			}

			response, err := adapter.Get(server.URL)

			if tt.expectError && err == nil {
				t.Fatal("expected error, got nil")
			}

			if !tt.expectError && err != nil {
				t.Fatalf("unexpected error: %v", err)
			}

			if tt.expectRead && !diskManager.readCalled {
				t.Fatal("expected disk manager ReadAll to be called")
			}

			if !tt.expectError {
				if response.StatusCode != tt.expectedStatus {
					t.Errorf("expected status %d, got %d",
						tt.expectedStatus, response.StatusCode)
				}

				if string(response.Body) != tt.expectedBody {
					t.Errorf("expected body %q, got %q",
						tt.expectedBody, string(response.Body))
				}
			}
		})
	}
}

func TestHttpAdapter_GetInvalidURL(t *testing.T) {
	diskManager := &MockDiskManager{}

	adapter := &HttpAdapter{
		diskManager: diskManager,
	}

	_, err := adapter.Get("://invalid-url")

	if err == nil {
		t.Fatal("expected error for invalid URL")
	}

	if diskManager.readCalled {
		t.Fatal("ReadAll should not be called when the HTTP request fails")
	}
}
