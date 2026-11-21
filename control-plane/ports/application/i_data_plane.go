package application

import "github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"

type IDataPlane interface {
	StartJob(service service.Service) error
}
