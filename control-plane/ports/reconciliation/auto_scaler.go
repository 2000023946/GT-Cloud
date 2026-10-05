package reconciliation

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/deployment"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/resource"
)

type Autoscaler interface {
	CalculateDesiredReplicas(
		deployment deployment.Deployment,
		measurements []resource.CPUMeasurement,
	) int
}
