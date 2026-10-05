package service

import (
	"testing"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/app"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/network"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/resource"
)

func TestService(t *testing.T) {
	s := Service{
		ID:       "service-1",
		Name:     "api",
		SystemID: "system-1",
		App: app.App{
			ID:       "app-1",
			CodePath: "/app",
			Command:  "python3 main.py",
		},
		Resources: resource.Resource{
			CPU:    2,
			Memory: 4096,
		},
		Network: []network.NetworkRule{
			{
				Protocol:  "TCP",
				Port:      8080,
				Direction: network.Inbound,
			},
		},
	}

	if s.ID != "service-1" {
		t.Errorf("expected ID service-1, got %s", s.ID)
	}

	if s.Name != "api" {
		t.Errorf("expected Name api, got %s", s.Name)
	}

	if s.SystemID != "system-1" {
		t.Errorf("expected SystemID system-1, got %s", s.SystemID)
	}

	if s.App.ID != "app-1" {
		t.Errorf("expected App ID app-1, got %s", s.App.ID)
	}

	if s.Resources.CPU != 2 {
		t.Errorf("expected CPU 2, got %d", s.Resources.CPU)
	}

	if s.Resources.Memory != 4096 {
		t.Errorf("expected Memory 4096, got %d", s.Resources.Memory)
	}

	if len(s.Network) != 1 {
		t.Errorf("expected 1 network rule, got %d", len(s.Network))
	}
}
