package get_job_logs

type GetJobLogsRequest struct {
	JobId string
}

func (s *GetJobLogsRequest) Validate() bool {
	return len(s.JobId) > 0
}
