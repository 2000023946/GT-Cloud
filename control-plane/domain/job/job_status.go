package job

type JobStatus string

const (
	StatusSubmitted JobStatus = "SUBMITTED"
	StatusQueued    JobStatus = "QUEUED"
	StatusScheduled JobStatus = "SCHEDULED"
	StatusRunning   JobStatus = "RUNNING"
	StatusCompleted JobStatus = "COMPLETED"
	StatusFailed    JobStatus = "FAILED"
	StatusCancelled JobStatus = "CANCELLED"
)
