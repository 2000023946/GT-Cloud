package execution

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/reconciliation"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/repository"
)

type DeploymentReconciler struct {
	queue                reconciliation.ReconciliationQueue
	deploymentRepo       repository.DeploymentRepository
	replicaRepo          repository.ReplicaRepository
	deploymentController reconciliation.DeploymentController
	actionExecutor       ActionExecutor
}
