package job_monitor

import (
	"fmt"
	"time"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports"
)

type JobMonitorService struct{}

func NewJobMonitorService() *JobMonitorService {
	return &JobMonitorService{}
}

func (m *JobMonitorService) Start(repository ports.JobRepository) {
	go func() {
		for {
			if err := m.CheckJobs(repository); err != nil {
				fmt.Println("JobMonitor:", err)
			}

			time.Sleep(5 * time.Second)
		}
	}()
}

func (m *JobMonitorService) CheckJobs(repository ports.JobRepository) error {
	fmt.Println("JobMonitor: CheckJobs")
	return nil
}
