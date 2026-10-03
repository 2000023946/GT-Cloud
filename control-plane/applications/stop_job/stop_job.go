package stop_job

import (
	"fmt"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports"
)

type StopJobService struct {
	jobRepository ports.JobRepository
	workerClient  ports.WorkerClient
}

func NewStopJobService(
	jobRepository ports.JobRepository,
	workerClient ports.WorkerClient,
) *StopJobService {
	return &StopJobService{
		jobRepository: jobRepository,
		workerClient:  workerClient,
	}
}

func (s *StopJobService) Stop() {
	fmt.Println("StopJob: Stop")
}
