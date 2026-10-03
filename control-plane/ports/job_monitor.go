package ports

type JobMonitor interface {
	Start(repository JobRepository)
	CheckJobs(repository JobRepository) error
}
