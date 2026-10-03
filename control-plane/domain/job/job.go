package job

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/app"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/network"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/resource"
)

type Job struct {
	ID        string
	App       app.App
	Status    JobStatus
	Resources resource.Resource
	Network   []network.NetworkRule
}
