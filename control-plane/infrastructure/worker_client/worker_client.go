package worker_client

import (
	"fmt"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/job"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/worker"
)

type WorkerClientService struct{}

func NewWorkerClientService() *WorkerClientService {
	return &WorkerClientService{}
}

func (c *WorkerClientService) RunJob(
	w worker.Worker,
	j job.Job,
) error {
	fmt.Println("WorkerClient: RunJob")
	return nil
}

func (c *WorkerClientService) StopJob(
	w worker.Worker,
	j job.Job,
) error {
	fmt.Println("WorkerClient: StopJob")
	return nil
}
