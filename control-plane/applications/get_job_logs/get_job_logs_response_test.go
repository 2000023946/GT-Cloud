package get_job_logs

import "testing"

func TestGetJobLogsResponseSuccess(t *testing.T) {
	stdout := "Job started successfully"
	stderr := ""

	response := GetJobLogsResponseSuccess(stdout, stderr)

	if response.isError {
		t.Errorf("expected isError to be false, got true")
	}

	if response.Message != "" {
		t.Errorf("expected empty Message, got %q", response.Message)
	}

	if response.stdout != stdout {
		t.Errorf("expected stdout %q, got %q", stdout, response.stdout)
	}

	if response.stderr != stderr {
		t.Errorf("expected stderr %q, got %q", stderr, response.stderr)
	}
}

func TestGetJobLogsResponseError(t *testing.T) {
	message := "failed to retrieve job logs"

	response := GetJobLogsResponseError(message)

	if !response.isError {
		t.Errorf("expected isError to be true, got false")
	}

	if response.Message != message {
		t.Errorf("expected Message %q, got %q", message, response.Message)
	}

	if response.stdout != "" {
		t.Errorf("expected empty stdout, got %q", response.stdout)
	}

	if response.stderr != "" {
		t.Errorf("expected empty stderr, got %q", response.stderr)
	}
}
