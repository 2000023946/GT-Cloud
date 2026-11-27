package get_jobs

import "github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"

type GetJobsResponse struct {
	Jobs    []JobDto
	IsError bool
	Message string
}

func GetJobsResponseSuccess(services []service.Service) GetJobsResponse {
	var job_dtos = CreateJobDtos(services)
	return GetJobsResponse{Jobs: job_dtos}
}

func GetJobsResponseError(message string) GetJobsResponse {
	return GetJobsResponse{IsError: true, Message: message}
}
