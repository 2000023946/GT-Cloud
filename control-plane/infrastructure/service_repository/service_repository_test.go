package service_repository

import (
	"fmt"
	"reflect"
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

func TestServiceRepository_Get(t *testing.T) {
	t.Run("successful retrieval", func(t *testing.T) {
		repository := NewServiceRepository()

		newService := service.Service{
			ID:   "service-1",
			Name: "test-service",
		}

		err := repository.Create(newService)

		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}

		result, err := repository.Get("service-1")

		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}

		if result.ID != newService.ID {
			t.Errorf("expected ID %s, got %s", newService.ID, result.ID)
		}

		if result.Name != newService.Name {
			t.Errorf("expected Name %s, got %s", newService.Name, result.Name)
		}
	})

	t.Run("service does not exist", func(t *testing.T) {
		repository := NewServiceRepository()

		result, err := repository.Get("missing-service")

		if err == nil {
			t.Fatal("expected error when service does not exist")
		}

		if !reflect.DeepEqual(result, service.Service{}) {
			t.Errorf("expected empty service, got %+v", result)
		}
	})
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

func TestServiceRepository_ConcurrentGet(t *testing.T) {
	repository := NewServiceRepository()

	const goroutineCount = 100

	for i := 0; i < goroutineCount; i++ {
		newService := service.Service{
			ID: fmt.Sprintf("service-%d", i),
		}

		if err := repository.Create(newService); err != nil {
			t.Fatalf("failed to create service: %v", err)
		}
	}

	var wg sync.WaitGroup

	for i := 0; i < goroutineCount; i++ {
		wg.Add(1)

		go func(id int) {
			defer wg.Done()

			expectedID := fmt.Sprintf("service-%d", id)

			result, err := repository.Get(expectedID)

			if err != nil {
				t.Errorf("failed to get service %s: %v", expectedID, err)
				return
			}

			if result.ID != expectedID {
				t.Errorf("expected ID %s, got %s", expectedID, result.ID)
			}
		}(i)
	}

	wg.Wait()
}

func TestServiceRepository_ConcurrentCreateAndGet(t *testing.T) {
	repository := NewServiceRepository()

	const goroutineCount = 100

	var wg sync.WaitGroup

	for i := 0; i < goroutineCount; i++ {
		wg.Add(1)

		go func(id int) {
			defer wg.Done()

			newService := service.Service{
				ID:   fmt.Sprintf("service-%d", id),
				Name: fmt.Sprintf("test-service-%d", id),
			}

			if err := repository.Create(newService); err != nil {
				t.Errorf("failed to create service: %v", err)
				return
			}

			result, err := repository.Get(newService.ID)

			if err != nil {
				t.Errorf("failed to get service %s: %v", newService.ID, err)
				return
			}

			if result.ID != newService.ID {
				t.Errorf("expected ID %s, got %s", newService.ID, result.ID)
			}

			if result.Name != newService.Name {
				t.Errorf("expected Name %s, got %s", newService.Name, result.Name)
			}
		}(i)
	}

	wg.Wait()

	if len(repository.storage) != goroutineCount {
		t.Errorf("expected %d services, got %d", goroutineCount, len(repository.storage))
	}
}

func TestServiceRepository_GetAll(t *testing.T) {
	repository := NewServiceRepository()

	services := []service.Service{
		{ID: "service-1", Name: "web-server"},
		{ID: "service-2", Name: "database"},
		{ID: "service-3", Name: "cache"},
	}

	for _, s := range services {
		if err := repository.Create(s); err != nil {
			t.Fatalf("failed to create service: %v", err)
		}
	}

	result, err := repository.GetAll()

	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if len(result) != len(services) {
		t.Fatalf("expected %d services, got %d", len(services), len(result))
	}

	expected := make(map[string]service.Service)

	for _, s := range services {
		expected[s.ID] = s
	}

	for _, actual := range result {
		expectedService, exists := expected[actual.ID]

		if !exists {
			t.Errorf("unexpected service ID: %s", actual.ID)
			continue
		}

		if actual.Name != expectedService.Name {
			t.Errorf("expected Name %s, got %s",
				expectedService.Name, actual.Name)
		}
	}
}

func TestServiceRepository_GetAllEmpty(t *testing.T) {
	repository := NewServiceRepository()

	result, err := repository.GetAll()

	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if len(result) != 0 {
		t.Errorf("expected 0 services, got %d", len(result))
	}
}

func TestServiceRepository_ConcurrentGetAll(t *testing.T) {
	repository := NewServiceRepository()

	const serviceCount = 100
	const goroutineCount = 100

	for i := 0; i < serviceCount; i++ {
		newService := service.Service{
			ID:   fmt.Sprintf("service-%d", i),
			Name: fmt.Sprintf("test-service-%d", i),
		}

		if err := repository.Create(newService); err != nil {
			t.Fatalf("failed to create service: %v", err)
		}
	}

	var wg sync.WaitGroup

	for i := 0; i < goroutineCount; i++ {
		wg.Add(1)

		go func() {
			defer wg.Done()

			result, err := repository.GetAll()

			if err != nil {
				t.Errorf("failed to get all services: %v", err)
				return
			}

			if len(result) != serviceCount {
				t.Errorf("expected %d services, got %d",
					serviceCount, len(result))
				return
			}

			seen := make(map[string]bool)

			for _, s := range result {
				if seen[s.ID] {
					t.Errorf("duplicate service ID: %s", s.ID)
				}

				seen[s.ID] = true
			}
		}()
	}

	wg.Wait()
}

func TestServiceRepository_ConcurrentCreateAndGetAll(t *testing.T) {
	repository := NewServiceRepository()

	const serviceCount = 100
	const readerCount = 20

	var wg sync.WaitGroup

	// Concurrent writers
	for i := 0; i < serviceCount; i++ {
		wg.Add(1)

		go func(id int) {
			defer wg.Done()

			newService := service.Service{
				ID:   fmt.Sprintf("service-%d", id),
				Name: fmt.Sprintf("test-service-%d", id),
			}

			if err := repository.Create(newService); err != nil {
				t.Errorf("failed to create service: %v", err)
			}
		}(i)
	}

	// Concurrent readers
	for i := 0; i < readerCount; i++ {
		wg.Add(1)

		go func() {
			defer wg.Done()

			for j := 0; j < 20; j++ {
				result, err := repository.GetAll()

				if err != nil {
					t.Errorf("failed to get all services: %v", err)
					return
				}

				// A snapshot can contain any number of services
				// between 0 and serviceCount during concurrent writes.
				if len(result) > serviceCount {
					t.Errorf("unexpected service count: %d", len(result))
				}

				seen := make(map[string]bool)

				for _, s := range result {
					if seen[s.ID] {
						t.Errorf("duplicate service ID: %s", s.ID)
					}

					seen[s.ID] = true
				}
			}
		}()
	}

	wg.Wait()

	// Verify final repository state after all writes finish.
	result, err := repository.GetAll()

	if err != nil {
		t.Fatalf("failed final GetAll: %v", err)
	}

	if len(result) != serviceCount {
		t.Errorf("expected %d services, got %d",
			serviceCount, len(result))
	}
}
