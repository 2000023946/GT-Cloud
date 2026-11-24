package disk_manager

import (
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestDiskManager_ReadAll(t *testing.T) {
	diskManager := &DiskManager{}

	t.Run("successful read", func(t *testing.T) {
		content := "hello GT-Cloud"

		reader := strings.NewReader(content)

		data, err := diskManager.ReadAll(reader)

		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}

		if string(data) != content {
			t.Errorf("expected %q, got %q", content, string(data))
		}
	})

	t.Run("reader error", func(t *testing.T) {
		expectedErr := errors.New("read failed")

		reader := &errorReader{err: expectedErr}

		data, err := diskManager.ReadAll(reader)

		if err == nil {
			t.Fatal("expected an error, got nil")
		}

		if !errors.Is(err, expectedErr) {
			t.Errorf("expected error %v, got %v", expectedErr, err)
		}

		if len(data) != 0 {
			t.Errorf("expected empty data, got %v", data)
		}
	})
}

func TestDiskManager_WriteFile(t *testing.T) {
	diskManager := &DiskManager{}

	t.Run("successful write", func(t *testing.T) {
		tempDir := t.TempDir()

		filePath := filepath.Join(tempDir, "test.txt")
		content := []byte("hello GT-Cloud")

		err := diskManager.WriteFile(filePath, content, 0644)

		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}

		data, err := os.ReadFile(filePath)

		if err != nil {
			t.Fatalf("failed to read written file: %v", err)
		}

		if string(data) != string(content) {
			t.Errorf("expected %q, got %q", content, data)
		}
	})

	t.Run("invalid path", func(t *testing.T) {
		tempDir := t.TempDir()

		filePath := filepath.Join(tempDir, "nonexistent", "test.txt")

		err := diskManager.WriteFile(filePath, []byte("test"), 0644)

		if err == nil {
			t.Fatal("expected an error, got nil")
		}
	})
}

type errorReader struct {
	err error
}

func (r *errorReader) Read(p []byte) (int, error) {
	return 0, r.err
}
