package ports

import "github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/worker"

type WorkerRepository interface {
	Save(worker worker.Worker) error
	Get(id string) (worker.Worker, error)
	Update(worker worker.Worker) error
	GetAll() ([]worker.Worker, error)
}
