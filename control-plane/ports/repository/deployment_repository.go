package repository

import "github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/deployment"

type DeploymentRepository interface {
	Create(deployment deployment.Deployment) error
	Get(id string) (deployment.Deployment, error)
	GetAll() ([]deployment.Deployment, error)
	Update(deployment deployment.Deployment) error
	Delete(id string) error
}
