package get_job_logs

type GetJobLogsResponse struct {
	isError bool
	Message string
	stdout  string
	stderr  string
}

func GetJobLogsResponseSuccess(stdout string, stderr string) GetJobLogsResponse {
	return GetJobLogsResponse{stdout: stdout, stderr: stderr, Message: "", isError: false}
}

func GetJobLogsResponseError(message string) GetJobLogsResponse {
	return GetJobLogsResponse{isError: true, Message: message, stdout: "", stderr: ""}
}
