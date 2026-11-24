package get_job_logs

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/application"
)

type GetJobLogsUseCase struct {
	DataPlane application.IDataPlane
}

func (s *GetJobLogsUseCase) Execute(request GetJobLogsRequest) GetJobLogsResponse {
	// validates the reqeust object
	if !request.Validate() {
		return GetJobLogsResponseError("Job Id cannot be empty!")
	}
	// calls the data plane
	jobLogs, err := s.DataPlane.GetJobLogs(request.JobId)
	if err != nil {
		return GetJobLogsResponseError("Job Logs could not be fetched!")
	}
	// creates the response based on the data plane result
	return GetJobLogsResponseSuccess(jobLogs.Stdout, jobLogs.Stderr)
}
