package worker_repository

import (
	"fmt"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/worker"
)

type InMemoryWorkerRepository struct{}

func NewInMemoryWorkerRepository() *InMemoryWorkerRepository {
	return &InMemoryWorkerRepository{}
}

func (r *InMemoryWorkerRepository) Save(w worker.Worker) error {
	fmt.Println("WorkerRepository: Save", w.ID)
	return nil
}

func (r *InMemoryWorkerRepository) Get(id string) (worker.Worker, error) {
	fmt.Println("WorkerRepository: Get", id)
	return worker.Worker{}, nil
}

func (r *InMemoryWorkerRepository) Update(w worker.Worker) error {
	fmt.Println("WorkerRepository: Update", w.ID)
	return nil
}

func (r *InMemoryWorkerRepository) GetAll() ([]worker.Worker, error) {
	fmt.Println("WorkerRepository: GetAll")
	return nil, nil
}
