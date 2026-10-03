package submit_job

import (
	"fmt"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports"
)

type SubmitJobService struct {
	jobRepository    ports.JobRepository
	workerRepository ports.WorkerRepository
	scheduler        ports.Scheduler
}

func NewSubmitJobService(
	jobRepository ports.JobRepository,
	workerRepository ports.WorkerRepository,
	scheduler ports.Scheduler,
) *SubmitJobService {
	return &SubmitJobService{
		jobRepository:    jobRepository,
		workerRepository: workerRepository,
		scheduler:        scheduler,
	}
}

func (s *SubmitJobService) Submit() {
	fmt.Println("SubmitJob: Submit")
}
