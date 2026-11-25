package get_job_resources

import (
	"testing"
)

func TestCreate(t *testing.T) {
	t.Run("empty job ID returns error", func(t *testing.T) {
		request, err := Create("")

		if err == nil {
			t.Fatal("expected an error for empty job ID")
		}

		if request.JobId != "" {
			t.Errorf("expected empty JobId, got %q", request.JobId)
		}
	})

	t.Run("valid job ID creates request", func(t *testing.T) {
		jobID := "job-123"

		request, err := Create(jobID)

		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}

		if request.JobId != jobID {
			t.Errorf("expected JobId %q, got %q", jobID, request.JobId)
		}
	})
}
