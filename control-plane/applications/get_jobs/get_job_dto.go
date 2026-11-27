package get_jobs

import "github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"

type JobDto struct {
	JobId string
	Name  string
}

func CreateJobDtos(services []service.Service) []JobDto {
	var job_list = make([]JobDto, 0)
	for _, curr_service := range services {
		var job = JobDto{JobId: curr_service.ID, Name: curr_service.Name}
		job_list = append(job_list, job)
	}
	return job_list
}
