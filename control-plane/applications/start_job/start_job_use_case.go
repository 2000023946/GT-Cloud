package start_job

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/app"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/resource"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/application"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/repository"
)

type StartJobUseCase struct {
	codeRepo    repository.ICodeRepository
	serviceRepo repository.IServiceRepository
	dataPlane   application.IDataPlane
}

func (s *StartJobUseCase) Execute(request StartJobRequest) (StartJobResponse, error) {
	// make sure none of the request objects is not empty or none
	err := request.IsValid()
	if err != nil {
		return StartJobResponseError(err), nil
	}
	// validate the resource + create Resource object
	newResource, err := resource.NewResource(float32(request.CpuResource), float32(request.MemoryResource))
	if err != nil {
		return StartJobResponseError(err), nil
	}
	// save to the code to the system and get codepath
	codepath, err := s.codeRepo.Save(request.CodeFile)
	if err != nil {
		return StartJobResponseError(err), nil
	}
	// create the App object
	newApp := app.CreateApp(codepath, request.StartCommand)
	// create the service object
	newService := service.CreateService(*newApp, *newResource, request.Name)
	// save the service object
	err = s.serviceRepo.Create(newService)
	if err != nil {
		return StartJobResponseError(err), nil
	}
	// send the request to the control plane
	err = s.dataPlane.StartJob(newService)
	if err != nil {
		return StartJobResponseError(err), nil
	}
	return StartJobResponseSuccess(), nil
}
