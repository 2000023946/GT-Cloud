package start_job

import (
	"mime/multipart"
	"testing"
)

type mockFile struct {
	filename string
	size     int
}

func (m *mockFile) Filename() string {
	return m.filename
}

func (m *mockFile) Open() (multipart.File, error) {
	return nil, nil
}

func (m *mockFile) Size() int {
	return m.size
}

func TestStartJobRequest_IsValid(t *testing.T) {
	tests := []struct {
		name    string
		request StartJobRequest
		wantErr bool
	}{
		{
			name: "valid request",
			request: StartJobRequest{
				Name:           "test-job",
				StartCommand:   "go run main.go",
				CodeFile:       &mockFile{filename: "main.go", size: 100},
				CpuResource:    2,
				MemoryResource: 512,
			},
			wantErr: false,
		},
		{
			name: "empty name",
			request: StartJobRequest{
				Name:         "",
				StartCommand: "go run main.go",
				CodeFile:     &mockFile{filename: "main.go", size: 100},
			},
			wantErr: true,
		},
		{
			name: "empty start command",
			request: StartJobRequest{
				Name:         "test-job",
				StartCommand: "",
				CodeFile:     &mockFile{filename: "main.go", size: 100},
			},
			wantErr: true,
		},
		{
			name: "nil code file",
			request: StartJobRequest{
				Name:         "test-job",
				StartCommand: "go run main.go",
				CodeFile:     nil,
			},
			wantErr: true,
		},
		{
			name: "empty code file",
			request: StartJobRequest{
				Name:         "test-job",
				StartCommand: "go run main.go",
				CodeFile:     &mockFile{filename: "main.go", size: 0},
			},
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.request.IsValid()

			if tt.wantErr {
				if err == nil {
					t.Fatal("expected an error, got nil")
				}
			} else {
				if err != nil {
					t.Fatalf("expected no error, got: %v", err)
				}
			}
		})
	}
}
