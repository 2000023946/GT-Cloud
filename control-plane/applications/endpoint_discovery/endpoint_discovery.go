package endpoint_discovery

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/endpoint"
)

type EndpointDiscovery interface {
	Resolve(serviceID string) ([]endpoint.Endpoint, error)
}
