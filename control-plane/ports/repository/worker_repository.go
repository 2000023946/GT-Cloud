package repository

import "github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/worker"

type WorkerRepository interface {
	Create(worker worker.Worker) error
	Get(id string) (worker.Worker, error)
	GetAll() ([]worker.Worker, error)
	Update(worker worker.Worker) error
	Delete(id string) error
}
