package repository

import "github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/system"

type SystemRepository interface {
	Create(system system.System) error
	Get(id string) (system.System, error)
	GetAll() ([]system.System, error)
	Update(system system.System) error
	Delete(id string) error
}
