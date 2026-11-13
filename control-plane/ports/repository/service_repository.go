package repository

import "github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"

type ServiceRepository interface {
	Create(service service.Service) error
	Get(id string) (service.Service, error)
	GetAll() ([]service.Service, error)
	Update(service service.Service) error
	Delete(id string) error
}
