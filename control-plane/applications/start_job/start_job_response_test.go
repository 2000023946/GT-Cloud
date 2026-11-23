package start_job

import (
	"errors"
	"testing"
)

func TestStartJobResponseError(t *testing.T) {
	expectedErr := errors.New("failed to start job")

	response := StartJobResponseError(expectedErr)

	if !response.IsError {
		t.Error("expected IsError to be true")
	}

	if response.Message != expectedErr.Error() {
		t.Errorf("expected message %q, got %q", expectedErr.Error(), response.Message)
	}
}

func TestStartJobResponseSuccess(t *testing.T) {
	response := StartJobResponseSuccess()

	if response.IsError {
		t.Error("expected IsError to be false")
	}

	expectedMessage := "Job has successfully been started"

	if response.Message != expectedMessage {
		t.Errorf("expected message %q, got %q", expectedMessage, response.Message)
	}
}
