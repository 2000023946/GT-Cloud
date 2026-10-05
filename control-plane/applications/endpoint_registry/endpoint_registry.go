package endpointregistry

import "github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/endpoint"

type EndpointRegistry interface {
	Register(endpoint endpoint.Endpoint) error
	Update(endpoint endpoint.Endpoint) error
	Remove(endpointID string) error
}
