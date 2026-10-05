package placement

import "testing"

func TestPlacement(t *testing.T) {
	p := Placement{
		ReplicaID: "replica-1",
		WorkerID:  "worker-1",
	}

	if p.ReplicaID != "replica-1" {
		t.Errorf("expected ReplicaID replica-1, got %s", p.ReplicaID)
	}

	if p.WorkerID != "worker-1" {
		t.Errorf("expected WorkerID worker-1, got %s", p.WorkerID)
	}
}
