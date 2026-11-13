package execution

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/deployment"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/placement"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/worker"
)

type Scheduler interface {
	Schedule(
		deployment deployment.Deployment,
		service service.Service,
		workers []worker.Worker,
	) ([]placement.Placement, error)
}
