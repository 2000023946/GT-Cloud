package json_adapter

import (
	"bytes"
	"testing"
)

func TestJsonAdapter_Marshal(t *testing.T) {
	adapter := &JsonAdapter{}

	t.Run("successful marshal", func(t *testing.T) {
		content := map[string]string{
			"name": "test-job",
		}

		result, err := adapter.Marshal(content)

		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}

		expected := []byte(`{"name":"test-job"}`)

		if !bytes.Equal(result, expected) {
			t.Errorf("expected %s, got %s", expected, result)
		}
	})

	t.Run("marshal error", func(t *testing.T) {
		content := make(chan int)

		result, err := adapter.Marshal(content)

		if err == nil {
			t.Fatal("expected an error, got nil")
		}

		if result != nil {
			t.Errorf("expected nil result, got %s", result)
		}
	})
}

func TestJsonAdapter_Unmarshal(t *testing.T) {
	adapter := &JsonAdapter{}

	t.Run("successful unmarshal", func(t *testing.T) {
		data := []byte(`{"name":"test-job","status":"running"}`)

		var result struct {
			Name   string `json:"name"`
			Status string `json:"status"`
		}

		err := adapter.Unmarshal(data, &result)

		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}

		if result.Name != "test-job" {
			t.Errorf("expected name test-job, got %s", result.Name)
		}

		if result.Status != "running" {
			t.Errorf("expected status running, got %s", result.Status)
		}
	})

	t.Run("invalid JSON", func(t *testing.T) {
		data := []byte(`{"name":`)

		var result map[string]string

		err := adapter.Unmarshal(data, &result)

		if err == nil {
			t.Fatal("expected an error for invalid JSON, got nil")
		}
	})

	t.Run("invalid destination", func(t *testing.T) {
		data := []byte(`{"name":"test-job"}`)

		var result struct {
			Name string `json:"name"`
		}

		err := adapter.Unmarshal(data, result)

		if err == nil {
			t.Fatal("expected an error for non-pointer destination, got nil")
		}
	})

	t.Run("empty JSON input", func(t *testing.T) {
		data := []byte{}

		var result map[string]string

		err := adapter.Unmarshal(data, &result)

		if err == nil {
			t.Fatal("expected an error for empty JSON input, got nil")
		}
	})
}
