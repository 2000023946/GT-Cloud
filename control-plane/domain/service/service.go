package service

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/app"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/network"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/resource"
)

type Service struct {
	ID              string
	Name            string
	App             app.App
	SystemID        string
	DesiredReplicas int
	Resources       resource.Resource
	Network         []network.NetworkRule
}
