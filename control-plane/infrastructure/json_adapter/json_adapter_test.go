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
