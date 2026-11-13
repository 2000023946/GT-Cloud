package repository

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/endpoint"
)

type EndpointRepository interface {
	Create(endpoint endpoint.Endpoint) error
	Get(id string) (endpoint.Endpoint, error)
	GetAll() ([]endpoint.Endpoint, error)
	Update(endpoint endpoint.Endpoint) error
	Delete(id string) error
}
