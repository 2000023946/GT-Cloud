package get_job_resources

import "testing"

func TestGetJobResourcesResponseSuccess(t *testing.T) {
	t.Run("success response contains correct values", func(t *testing.T) {
		cpuUsed := float32(25.5)
		memoryUsed := float32(512.75)
		cpuMax := float32(100)
		memoryMax := float32(1024)

		response := GetJobResourcesResponseSuccess(
			cpuUsed,
			memoryUsed,
			cpuMax,
			memoryMax,
		)

		if response.CpuUsed != cpuUsed {
			t.Errorf("expected CpuUsed %v, got %v", cpuUsed, response.CpuUsed)
		}

		if response.MemoryUsed != memoryUsed {
			t.Errorf("expected MemoryUsed %v, got %v", memoryUsed, response.MemoryUsed)
		}

		if response.CpuMax != cpuMax {
			t.Errorf("expected CpuMax %v, got %v", cpuMax, response.CpuMax)
		}

		if response.MemoryMax != memoryMax {
			t.Errorf("expected MemoryMax %v, got %v", memoryMax, response.MemoryMax)
		}

		if response.IsError {
			t.Error("expected IsError to be false")
		}

		if response.Message != "" {
			t.Errorf("expected empty Message, got %q", response.Message)
		}
	})
}

func TestGetJobResourcesResponseError(t *testing.T) {
	t.Run("error response contains correct values", func(t *testing.T) {
		message := "failed to retrieve job resources"

		response := GetJobResourcesResponseError(message)

		if !response.IsError {
			t.Error("expected IsError to be true")
		}

		if response.Message != message {
			t.Errorf("expected Message %q, got %q", message, response.Message)
		}
	})
}
