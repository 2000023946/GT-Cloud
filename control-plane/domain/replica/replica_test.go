package replica

import "testing"

func TestReplica(t *testing.T) {
	r := Replica{
		ID:           "replica-1",
		DeploymentID: "deployment-1",
		WorkerID:     "worker-1",
		Status:       Running,
	}

	if r.ID != "replica-1" {
		t.Errorf("expected ID replica-1, got %s", r.ID)
	}

	if r.DeploymentID != "deployment-1" {
		t.Errorf("expected DeploymentID deployment-1, got %s", r.DeploymentID)
	}

	if r.WorkerID != "worker-1" {
		t.Errorf("expected WorkerID worker-1, got %s", r.WorkerID)
	}

	if r.Status != Running {
		t.Errorf("expected status RUNNING, got %s", r.Status)
	}
}
