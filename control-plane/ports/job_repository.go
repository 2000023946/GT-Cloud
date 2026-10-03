package ports

import "github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/job"

type JobRepository interface {
	Save(job job.Job) error
	Get(id string) (job.Job, error)
	Update(job job.Job) error
	GetRunningJobs() ([]job.Job, error)
}
