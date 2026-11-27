package stop_job

type StopJobResponse struct {
	IsSuccess bool
	IsError   bool
	Message   string
}

func StopJobResponseSuccess() StopJobResponse {
	return StopJobResponse{IsSuccess: true, IsError: false, Message: ""}
}

func StopJobResponseError(message string) StopJobResponse {
	return StopJobResponse{IsSuccess: false, IsError: true, Message: message}
}
