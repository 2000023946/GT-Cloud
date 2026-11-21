package service_repository

import (
	"fmt"
	"sync"
	"testing"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
)

func TestNewServiceRepository(t *testing.T) {
	repository := NewServiceRepository()

	if repository == nil {
		t.Fatal("expected repository, got nil")
	}

	if repository.storage == nil {
		t.Fatal("expected initialized storage map")
	}
}

func TestServiceRepository_Create(t *testing.T) {
	repository := NewServiceRepository()

	newService := service.Service{
		ID: "service-1",
	}

	err := repository.Create(newService)

	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	storedService, exists := repository.storage["service-1"]

	if !exists {
		t.Fatal("expected service to be stored")
	}

	if storedService.ID != newService.ID {
		t.Errorf("expected ID %s, got %s", newService.ID, storedService.ID)
	}
}

func TestServiceRepository_ConcurrentCreate(t *testing.T) {
	repository := NewServiceRepository()

	const goroutineCount = 100

	var wg sync.WaitGroup

	for i := 0; i < goroutineCount; i++ {
		wg.Add(1)

		go func(id int) {
			defer wg.Done()

			newService := service.Service{
				ID: fmt.Sprintf("service-%d", id),
			}

			if err := repository.Create(newService); err != nil {
				t.Errorf("failed to create service: %v", err)
			}
		}(i)
	}

	wg.Wait()

	if len(repository.storage) != goroutineCount {
		t.Errorf("expected %d services, got %d", goroutineCount, len(repository.storage))
	}

	for i := 0; i < goroutineCount; i++ {
		id := fmt.Sprintf("service-%d", i)

		if _, exists := repository.storage[id]; !exists {
			t.Errorf("expected service %s to exist", id)
		}
	}
}

func TestServiceRepository_ConcurrentCreateSameID(t *testing.T) {
	repository := NewServiceRepository()

	const goroutineCount = 100

	var wg sync.WaitGroup

	for i := 0; i < goroutineCount; i++ {
		wg.Add(1)

		go func() {
			defer wg.Done()

			newService := service.Service{
				ID: "shared-service",
			}

			if err := repository.Create(newService); err != nil {
				t.Errorf("failed to create service: %v", err)
			}
		}()
	}

	wg.Wait()

	if len(repository.storage) != 1 {
		t.Errorf("expected 1 service, got %d", len(repository.storage))
	}

	if _, exists := repository.storage["shared-service"]; !exists {
		t.Error("expected shared-service to exist")
	}
}
