package job

import (
	"testing"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/app"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/network"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/resource"
)

func TestJob(t *testing.T) {
	j := Job{
		ID: "job-1",

		App: app.App{
			ID:       "app-1",
			CodePath: "/apps/app-1/",
			Command:  "python3 main.py",
		},

		Status: StatusSubmitted,

		Resources: resource.Resource{
			CPU:    4,
			Memory: 8 * 1024 * 1024 * 1024,
		},

		Network: []network.NetworkRule{
			{
				Protocol:  "TCP",
				Port:      8080,
				Direction: network.Inbound,
			},
		},
	}

	if j.ID != "job-1" {
		t.Errorf("expected job ID job-1, got %s", j.ID)
	}

	if j.App.ID != "app-1" {
		t.Errorf("expected app ID app-1, got %s", j.App.ID)
	}

	if j.App.CodePath != "/apps/app-1/" {
		t.Errorf("expected code path /apps/app-1/, got %s", j.App.CodePath)
	}

	if j.App.Command != "python3 main.py" {
		t.Errorf("expected command python3 main.py, got %s", j.App.Command)
	}

	if j.Status != StatusSubmitted {
		t.Errorf("expected status SUBMITTED, got %s", j.Status)
	}

	if j.Resources.CPU != 4 {
		t.Errorf("expected CPU 4, got %d", j.Resources.CPU)
	}

	if j.Resources.Memory != 8*1024*1024*1024 {
		t.Errorf("expected memory 8 GB, got %d", j.Resources.Memory)
	}

	if len(j.Network) != 1 {
		t.Fatalf("expected 1 network rule, got %d", len(j.Network))
	}

	if j.Network[0].Protocol != "TCP" {
		t.Errorf("expected protocol TCP, got %s", j.Network[0].Protocol)
	}

	if j.Network[0].Port != 8080 {
		t.Errorf("expected port 8080, got %d", j.Network[0].Port)
	}

	if j.Network[0].Direction != network.Inbound {
		t.Errorf("expected direction INBOUND, got %s", j.Network[0].Direction)
	}
}
