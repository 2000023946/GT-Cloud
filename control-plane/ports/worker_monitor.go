package ports

type WorkerMonitor interface {
	Start(repository WorkerRepository)
	CheckWorkers(repository WorkerRepository) error
}
