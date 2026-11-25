package get_job_resources

import "errors"

type GetJobResourcesRequest struct {
	JobId string
}

func Create(job_id string) (GetJobResourcesRequest, error) {
	if len(job_id) == 0 {
		return GetJobResourcesRequest{}, errors.New("Job Id cannot be empty!")
	}
	return GetJobResourcesRequest{JobId: job_id}, nil
}
