package ports

type SelfHealer interface {
	Start(jobRepository JobRepository, workerRepository WorkerRepository)
	Heal(jobRepository JobRepository, workerRepository WorkerRepository) error
}
