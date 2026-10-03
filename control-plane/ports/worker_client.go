package ports

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/job"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/worker"
)

type WorkerClient interface {
	RunJob(worker worker.Worker, job job.Job) error
	StopJob(worker worker.Worker, job job.Job) error
}
