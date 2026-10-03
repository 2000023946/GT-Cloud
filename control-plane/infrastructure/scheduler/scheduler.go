package scheduler

import (
	"fmt"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/job"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/worker"
)

type SchedulerService struct{}

func NewSchedulerService() *SchedulerService {
	return &SchedulerService{}
}

func (s *SchedulerService) Schedule(
	j job.Job,
	workers []worker.Worker,
) (worker.Worker, error) {
	fmt.Println("Scheduler: Schedule")
	return worker.Worker{}, nil
}
