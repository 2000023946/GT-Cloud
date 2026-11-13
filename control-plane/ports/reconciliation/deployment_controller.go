package reconciliation

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/action"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/deployment"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/replica"
)

type DeploymentController interface {
	Reconcile(
		deployment deployment.Deployment,
		replicas []replica.Replica,
	) []action.Action
}
