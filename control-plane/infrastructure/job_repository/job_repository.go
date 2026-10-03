package job_repository

import (
	"fmt"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/job"
)

type InMemoryJobRepository struct{}

func NewInMemoryJobRepository() *InMemoryJobRepository {
	return &InMemoryJobRepository{}
}

func (r *InMemoryJobRepository) Save(j job.Job) error {
	fmt.Println("JobRepository: Save", j.ID)
	return nil
}

func (r *InMemoryJobRepository) Get(id string) (job.Job, error) {
	fmt.Println("JobRepository: Get", id)
	return job.Job{}, nil
}

func (r *InMemoryJobRepository) Update(j job.Job) error {
	fmt.Println("JobRepository: Update", j.ID)
	return nil
}

func (r *InMemoryJobRepository) GetRunningJobs() ([]job.Job, error) {
	fmt.Println("JobRepository: GetRunningJobs")
	return nil, nil
}
