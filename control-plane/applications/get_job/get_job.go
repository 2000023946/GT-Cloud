package get_job

import (
	"fmt"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports"
)

type GetJobService struct {
	jobRepository ports.JobRepository
}

func NewGetJobService(
	jobRepository ports.JobRepository,
) *GetJobService {
	return &GetJobService{
		jobRepository: jobRepository,
	}
}

func (s *GetJobService) Get() {
	fmt.Println("GetJob: Get")
}
