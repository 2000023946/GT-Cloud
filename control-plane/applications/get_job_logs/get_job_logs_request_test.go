package get_job_logs

import "testing"

func TestGetJobLogsRequestValidate(t *testing.T) {
	tests := []struct {
		name     string
		jobId    string
		expected bool
	}{
		{
			name:     "valid job ID",
			jobId:    "job-123",
			expected: true,
		},
		{
			name:     "empty job ID",
			jobId:    "",
			expected: false,
		},
		{
			name:     "job ID with whitespace",
			jobId:    "   ",
			expected: true, // Change to false if you trim whitespace.
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			request := GetJobLogsRequest{
				JobId: tt.jobId,
			}

			result := request.Validate()

			if result != tt.expected {
				t.Errorf("Validate() = %v, expected %v", result, tt.expected)
			}
		})
	}
}
