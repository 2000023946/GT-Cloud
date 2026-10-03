package ports

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/job"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/worker"
)

type Scheduler interface {
	Schedule(job job.Job, workers []worker.Worker) (worker.Worker, error)
}
