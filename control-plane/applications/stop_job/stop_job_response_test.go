package stop_job

import "testing"

func TestStopJobResponseSuccess(t *testing.T) {
	response := StopJobResponseSuccess()

	if !response.IsSuccess {
		t.Errorf("expected IsSuccess to be true, got %t", response.IsSuccess)
	}

	if response.IsError {
		t.Errorf("expected IsError to be false, got %t", response.IsError)
	}

	if response.Message != "" {
		t.Errorf("expected Message to be empty, got %q", response.Message)
	}
}

func TestStopJobResponseError(t *testing.T) {
	response := StopJobResponseError("error")

	if response.IsSuccess {
		t.Errorf("expected IsSuccess to be false, got %t", response.IsSuccess)
	}

	if !response.IsError {
		t.Errorf("expected IsError to be true, got %t", response.IsError)
	}

	if response.Message != "error" {
		t.Errorf("expected Message to be empty, got %q", response.Message)
	}
}
