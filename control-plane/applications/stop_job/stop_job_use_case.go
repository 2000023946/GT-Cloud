package stop_job

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/application"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/repository"
)

type StopJobUseCase struct {
	serviceRepo repository.IServiceRepository
	dataPlane   application.IDataPlane
}

func (s *StopJobUseCase) execute(request StopJobRequest) StopJobResponse {
	// send delete signal to data plane
	err := s.dataPlane.DeleteJob(request.JobID)
	if err != nil {
		return StopJobResponseError(err.Error())
	}
	// delete repo
	err = s.serviceRepo.Delete(request.JobID)
	if err != nil {
		return StopJobResponseError(err.Error())
	}
	// return response
	return StopJobResponseSuccess()

}
