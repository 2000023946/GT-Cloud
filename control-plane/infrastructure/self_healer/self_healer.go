package self_healer

import (
	"fmt"
	"time"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports"
)

type SelfHealerService struct{}

func NewSelfHealerService() *SelfHealerService {
	return &SelfHealerService{}
}

func (s *SelfHealerService) Start(
	jobRepository ports.JobRepository,
	workerRepository ports.WorkerRepository,
) {
	go func() {
		for {
			if err := s.Heal(jobRepository, workerRepository); err != nil {
				fmt.Println("SelfHealer:", err)
			}

			time.Sleep(5 * time.Second)
		}
	}()
}

func (s *SelfHealerService) Heal(
	jobRepository ports.JobRepository,
	workerRepository ports.WorkerRepository,
) error {
	fmt.Println("SelfHealer: Heal")
	return nil
}
