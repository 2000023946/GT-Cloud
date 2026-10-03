package worker_monitor

import (
	"fmt"
	"time"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports"
)

type WorkerMonitorService struct{}

func NewWorkerMonitorService() *WorkerMonitorService {
	return &WorkerMonitorService{}
}

func (m *WorkerMonitorService) Start(repository ports.WorkerRepository) {
	go func() {
		for {
			if err := m.CheckWorkers(repository); err != nil {
				fmt.Println("WorkerMonitor:", err)
			}

			time.Sleep(5 * time.Second)
		}
	}()
}

func (m *WorkerMonitorService) CheckWorkers(repository ports.WorkerRepository) error {
	fmt.Println("WorkerMonitor: CheckWorkers")
	return nil
}
