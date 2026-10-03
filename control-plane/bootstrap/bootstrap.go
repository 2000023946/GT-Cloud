package bootstrap

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/api"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/applications/get_job"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/applications/stop_job"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/applications/submit_job"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/infrastructure/job_monitor"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/infrastructure/job_repository"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/infrastructure/scheduler"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/infrastructure/self_healer"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/infrastructure/worker_client"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/infrastructure/worker_monitor"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/infrastructure/worker_repository"
)

func Start() *api.API {
	// Repositories
	jobRepository := job_repository.NewInMemoryJobRepository()
	workerRepository := worker_repository.NewInMemoryWorkerRepository()

	// Infrastructure services
	scheduler := scheduler.NewSchedulerService()
	workerClient := worker_client.NewWorkerClientService()

	jobMonitor := job_monitor.NewJobMonitorService()
	workerMonitor := worker_monitor.NewWorkerMonitorService()
	selfHealer := self_healer.NewSelfHealerService()

	// Application services
	submitJob := submit_job.NewSubmitJobService(
		jobRepository,
		workerRepository,
		scheduler,
	)

	getJob := get_job.NewGetJobService(
		jobRepository,
	)

	stopJob := stop_job.NewStopJobService(
		jobRepository,
		workerClient,
	)

	// Start background services
	jobMonitor.Start(jobRepository)
	workerMonitor.Start(workerRepository)

	selfHealer.Start(
		jobRepository,
		workerRepository,
	)

	// Create control-plane API
	controlPlaneAPI := api.NewAPI(
		submitJob,
		getJob,
		stopJob,
	)

	return controlPlaneAPI
}
