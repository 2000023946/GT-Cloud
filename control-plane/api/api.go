package api

import (
	"fmt"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/applications/get_job"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/applications/stop_job"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/applications/submit_job"
)

type API struct {
	submitJob *submit_job.SubmitJobService
	getJob    *get_job.GetJobService
	stopJob   *stop_job.StopJobService
}

func NewAPI(
	submitJob *submit_job.SubmitJobService,
	getJob *get_job.GetJobService,
	stopJob *stop_job.StopJobService,
) *API {
	return &API{
		submitJob: submitJob,
		getJob:    getJob,
		stopJob:   stopJob,
	}
}

func (a *API) SubmitJob() {
	fmt.Println("API: SubmitJob")
	a.submitJob.Submit()
}

func (a *API) GetJob() {
	fmt.Println("API: GetJob")
	a.getJob.Get()
}

func (a *API) StopJob() {
	fmt.Println("API: StopJob")
	a.stopJob.Stop()
}
