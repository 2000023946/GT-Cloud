package code_repository

import (
	"bytes"
	"errors"
	"io"
	"mime/multipart"
	"os"
	"strings"
	"testing"
)

// Mock IFile
type mockFile struct {
	filename string
	content  []byte
	openErr  error
}

func (m *mockFile) Filename() string {
	return m.filename
}

func (m *mockFile) Open() (multipart.File, error) {
	if m.openErr != nil {
		return nil, m.openErr
	}
	return &mockMultipartFile{Reader: bytes.NewReader(m.content)}, nil
}

func (m *mockFile) Size() int {
	return len(m.content)
}

// Mock multipart.File
type mockMultipartFile struct {
	*bytes.Reader
}

func (m *mockMultipartFile) Close() error {
	return nil
}

// Mock IDiskManager
type mockDiskManager struct {
	readErr  error
	writeErr error

	readContent []byte

	writtenPath string
	writtenData []byte
	writtenPerm os.FileMode
}

func (m *mockDiskManager) ReadAll(r io.Reader) ([]byte, error) {
	if m.readErr != nil {
		return nil, m.readErr
	}

	return io.ReadAll(r)
}

func (m *mockDiskManager) WriteFile(name string, data []byte, perm os.FileMode) error {
	m.writtenPath = name
	m.writtenData = data
	m.writtenPerm = perm

	return m.writeErr
}

// Tests
func TestCodeRepository_Save(t *testing.T) {
	t.Run("successful save", func(t *testing.T) {
		content := []byte("package main")

		diskManager := &mockDiskManager{}
		repository := CodeRepository{
			diskManager: diskManager,
		}

		codeFile := &mockFile{
			filename: "main.go",
			content:  content,
		}

		path, err := repository.Save(codeFile)

		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}

		if !strings.HasPrefix(path, "job/") {
			t.Errorf("expected path to start with job/, got %s", path)
		}

		if !strings.HasSuffix(path, "/main.go") {
			t.Errorf("expected path to end with /main.go, got %s", path)
		}

		if diskManager.writtenPath != path {
			t.Errorf("expected written path %s, got %s", path, diskManager.writtenPath)
		}

		if !bytes.Equal(diskManager.writtenData, content) {
			t.Errorf("expected written content %s, got %s", content, diskManager.writtenData)
		}

		if diskManager.writtenPerm != 0644 {
			t.Errorf("expected permission 0644, got %v", diskManager.writtenPerm)
		}
	})

	t.Run("file open error", func(t *testing.T) {
		expectedErr := errors.New("failed to open file")

		diskManager := &mockDiskManager{}
		repository := CodeRepository{
			diskManager: diskManager,
		}

		codeFile := &mockFile{
			filename: "main.go",
			openErr:  expectedErr,
		}

		path, err := repository.Save(codeFile)

		if err != expectedErr {
			t.Errorf("expected error %v, got %v", expectedErr, err)
		}

		if path != "" {
			t.Errorf("expected empty path, got %s", path)
		}
	})

	t.Run("read file error", func(t *testing.T) {
		expectedErr := errors.New("failed to read file")

		diskManager := &mockDiskManager{
			readErr: expectedErr,
		}

		repository := CodeRepository{
			diskManager: diskManager,
		}

		codeFile := &mockFile{
			filename: "main.go",
			content:  []byte("package main"),
		}

		path, err := repository.Save(codeFile)

		if err != expectedErr {
			t.Errorf("expected error %v, got %v", expectedErr, err)
		}

		if path != "" {
			t.Errorf("expected empty path, got %s", path)
		}
	})

	t.Run("write file error", func(t *testing.T) {
		expectedErr := errors.New("failed to write file")

		diskManager := &mockDiskManager{
			writeErr: expectedErr,
		}

		repository := CodeRepository{
			diskManager: diskManager,
		}

		codeFile := &mockFile{
			filename: "main.go",
			content:  []byte("package main"),
		}

		path, err := repository.Save(codeFile)

		if err != expectedErr {
			t.Errorf("expected error %v, got %v", expectedErr, err)
		}

		if path != "" {
			t.Errorf("expected empty path, got %s", path)
		}
	})
}
