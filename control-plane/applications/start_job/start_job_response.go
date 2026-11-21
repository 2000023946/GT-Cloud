package start_job

type StartJobResponse struct {
	IsError bool
	Message string
}

func StartJobResponseError(err error) StartJobResponse {
	return StartJobResponse{
		IsError: true,
		Message: err.Error(),
	}
}

func StartJobResponseSuccess() StartJobResponse {
	return StartJobResponse{
		IsError: false,
		Message: "Job has successfully been started",
	}
}
