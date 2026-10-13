package execution

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/action"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/repository"
)

type ActionExecutor interface {
	Execute(
		action action.Action,
		scheduler Scheduler,
		replica_repository repository.ReplicaRepository,
		worker_client WorkerClient,
	) error
}
