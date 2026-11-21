package file

import (
	"bytes"
	"io"
	"mime/multipart"
	"testing"
)

func TestFile_FileName(t *testing.T) {
	file := &File{
		File: &multipart.FileHeader{
			Filename: "main.go",
		},
	}

	result := file.FileName()

	if result != "main.go" {
		t.Errorf("expected main.go, got %s", result)
	}
}

func TestFile_Size(t *testing.T) {
	file := &File{
		File: &multipart.FileHeader{
			Size: 1024,
		},
	}

	result := file.Size()

	if result != 1024 {
		t.Errorf("expected 1024, got %d", result)
	}
}

func TestFile_Open(t *testing.T) {
	t.Run("successful open", func(t *testing.T) {
		var body bytes.Buffer
		writer := multipart.NewWriter(&body)

		part, err := writer.CreateFormFile("code", "main.go")
		if err != nil {
			t.Fatalf("failed to create form file: %v", err)
		}

		content := []byte("package main")
		if _, err := part.Write(content); err != nil {
			t.Fatalf("failed to write content: %v", err)
		}

		if err := writer.Close(); err != nil {
			t.Fatalf("failed to close writer: %v", err)
		}

		reader := multipart.NewReader(&body, writer.Boundary())

		form, err := reader.ReadForm(1024 * 1024)
		if err != nil {
			t.Fatalf("failed to read multipart form: %v", err)
		}
		defer form.RemoveAll()

		fileHeader := form.File["code"][0]

		file := &File{
			File: fileHeader,
		}

		openedFile, err := file.Open()
		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}
		defer openedFile.Close()

		data, err := io.ReadAll(openedFile)
		if err != nil {
			t.Fatalf("failed to read opened file: %v", err)
		}

		if !bytes.Equal(data, content) {
			t.Errorf("expected %q, got %q", content, data)
		}
	})
}
