package get_jobs

import "github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/repository"

type GetJobsUseCase struct {
	serviceRepo repository.IServiceRepository
}

func (s *GetJobsUseCase) execute() GetJobsResponse {
	// calls the service repo and then returns if there is error returns proper response object
	var services_fetched, err = s.serviceRepo.GetAll()
	if err != nil {
		return GetJobsResponseError(err.Error())
	}
	return GetJobsResponseSuccess(services_fetched)
}
