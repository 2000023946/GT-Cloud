package data_plane

import (
	"bytes"
	"errors"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/globals"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/job_logs"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/resource"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/application"
)

type DataPlane struct {
	jsonAdapter application.IJsonAdapter
	httpAdapter application.IHttpAdapter
}

func (s *DataPlane) StartJob(service service.Service) error {

	// load and send the data
	payload, err := s.jsonAdapter.Marshal(service)
	if err != nil {
		return err
	}
	// don't really care about the reqeust object
	_, err = s.httpAdapter.Post(
		globals.DataPlaneUrl, "application/json", bytes.NewBuffer(payload))
	if err != nil {
		return err
	}
	return nil
}

func (s *DataPlane) GetJobLogs(jobId string) (job_logs.JobLogs, error) {
	// makes http request to the data plane
	result, err := s.httpAdapter.Get(globals.DataPlaneUrl + "/job_logs/" + jobId)
	if err != nil {
		return job_logs.JobLogs{}, errors.New("Error fetching the job logs!")
	}
	// decode the object
	var jobLogs job_logs.JobLogs
	err = s.jsonAdapter.Unmarshal(result.Body, &jobLogs)
	if err != nil {
		return job_logs.JobLogs{}, errors.New("Error converting response bytes to Job Logs object")
	}
	return jobLogs, nil
}

func (s *DataPlane) GetJobResources(job_id string) (resource.Resource, error) {
	result, err := s.httpAdapter.Get(globals.DataPlaneUrl + "/job_resources/" + job_id)
	if err != nil {
		return resource.Resource{}, errors.New("Error fetching the job logs!")
	}

	// unpack the resource
	var res resource.Resource
	err = s.jsonAdapter.Unmarshal(result.Body, &res)
	if err != nil {
		return resource.Resource{}, errors.New("Error converting response bytes to Job Logs object")
	}

	return res, nil
}

func (s *DataPlane) DeleteJob(job_id string) error {
	_, err := s.httpAdapter.Get(globals.DataPlaneUrl + "/delete_job/" + job_id)
	if err != nil {
		return errors.New("Error deleting the job!")
	}
	return nil
}
