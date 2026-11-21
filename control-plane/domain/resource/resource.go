package resource

import (
	"errors"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/globals"
)

type Resource struct {
	CPU    float32
	Memory float32
}

func NewResource(cpu float32, memory float32) (*Resource, error) {
	if cpu <= 0 {
		return nil, errors.New("CPU count must be greater than 0")
	}
	if memory <= 0 {
		return nil, errors.New("Memory usage must be greater than 0")
	}
	if cpu > globals.MaxCPU {
		return nil, errors.New("CPU count exceeds maximum allowed CPU")
	}
	if memory > globals.MaxMemory {
		return nil, errors.New("Memory usage exceeds maximum allowed memory")
	}

	return &Resource{
		CPU:    cpu,
		Memory: memory,
	}, nil
}
