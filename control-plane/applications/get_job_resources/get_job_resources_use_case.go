package get_job_resources

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/application"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/repository"
)

type GetJobResourcesUseCase struct {
	service_repository repository.IServiceRepository
	data_plane         application.IDataPlane
}

func (s *GetJobResourcesUseCase) Create(request GetJobResourcesRequest) GetJobResourcesResponse {
	// get the job
	service, err := s.service_repository.Get(request.JobId)
	// throw error response if not exist
	if err != nil {
		return GetJobResourcesResponseError(err.Error())
	}
	// call the data plane to get the resources
	resource, err := s.data_plane.GetJobResources(request.JobId)
	// if response if error throw error response
	if err != nil {
		return GetJobResourcesResponseError(err.Error())
	}
	// return success reponse
	return GetJobResourcesResponseSuccess(
		resource.CPU, resource.Memory, service.Resources.CPU, service.Resources.Memory,
	)
}
