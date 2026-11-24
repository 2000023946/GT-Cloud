package service_repository

import (
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
