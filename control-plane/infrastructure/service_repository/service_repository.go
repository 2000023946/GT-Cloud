package service_repository

import (
	"fmt"
	"sync"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
)

type ServiceRepository struct {
	mu      sync.Mutex
	storage map[string]service.Service
}

func NewServiceRepository() *ServiceRepository {
	return &ServiceRepository{
		storage: make(map[string]service.Service),
	}
}

func (s *ServiceRepository) Create(service service.Service) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.storage[service.ID] = service
	return nil
}

func (s *ServiceRepository) Get(job_id string) (service.Service, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	job, ok := s.storage[job_id]
	if !ok {
		return service.Service{}, fmt.Errorf("job %q not found in repository", job_id) // or a sentinel error like ErrNotFound
	}
	return job, nil
}
