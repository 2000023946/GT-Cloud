package resource

import (
	"testing"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/globals"
)

func TestNewResource(t *testing.T) {
	tests := []struct {
		name    string
		cpu     float32
		memory  float32
		wantErr bool
	}{
		{
			name:    "valid resource",
			cpu:     2,
			memory:  512,
			wantErr: false,
		},
		{
			name:    "zero CPU",
			cpu:     0,
			memory:  512,
			wantErr: true,
		},
		{
			name:    "negative CPU",
			cpu:     -1,
			memory:  512,
			wantErr: true,
		},
		{
			name:    "zero memory",
			cpu:     2,
			memory:  0,
			wantErr: true,
		},
		{
			name:    "negative memory",
			cpu:     2,
			memory:  -1,
			wantErr: true,
		},
		{
			name:    "CPU exceeds maximum",
			cpu:     globals.MaxCPU + 1,
			memory:  512,
			wantErr: true,
		},
		{
			name:    "memory exceeds maximum",
			cpu:     2,
			memory:  globals.MaxMemory + 1,
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			resource, err := NewResource(tt.cpu, tt.memory)

			if tt.wantErr {
				if err == nil {
					t.Fatal("expected an error, got nil")
				}

				if resource != nil {
					t.Error("expected resource to be nil")
				}
			} else {
				if err != nil {
					t.Fatalf("expected no error, got: %v", err)
				}

				if resource == nil {
					t.Fatal("expected resource, got nil")
				}

				if resource.CPU != tt.cpu {
					t.Errorf("expected CPU %v, got %v", tt.cpu, resource.CPU)
				}

				if resource.Memory != tt.memory {
					t.Errorf("expected Memory %v, got %v", tt.memory, resource.Memory)
				}
			}
		})
	}
}
