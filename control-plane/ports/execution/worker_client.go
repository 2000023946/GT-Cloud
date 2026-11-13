package execution

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/replica"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/worker"
)

type WorkerClient interface {
	StartReplica(
		service service.Service,
		replica replica.Replica,
		worker worker.Worker,
	) error

	StopReplica(
		replica replica.Replica,
		worker worker.Worker,
	) error
}
