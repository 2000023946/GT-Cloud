package application

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/endpoint"
)

type EndpointSelector interface {
	Select(endpoints []endpoint.Endpoint) (endpoint.Endpoint, error)
}
