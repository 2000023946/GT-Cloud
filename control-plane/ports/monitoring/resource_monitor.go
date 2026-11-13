package monitoring

import "github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/resource"

type ResourceMonitor interface {
	GetReplicaCPUUsages(
		deploymentID string,
	) ([]resource.CPUMeasurement, error)
}
