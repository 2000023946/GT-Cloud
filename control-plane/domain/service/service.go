package service

import (
	"github.com/google/uuid"
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

func CreateService(newApp app.App, resources resource.Resource, name string) Service {
	return Service{
		ID:              uuid.New().String(),
		Name:            name,
		App:             newApp,
		SystemID:        uuid.New().String(),
		DesiredReplicas: 1,
		Resources:       resources,
		Network:         []network.NetworkRule{},
	}
}
