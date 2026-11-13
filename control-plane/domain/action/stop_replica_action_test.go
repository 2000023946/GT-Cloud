package action

import "testing"

func TestStopReplicaAction(t *testing.T) {
	action := StopReplicaAction{
		ReplicaID: "replica-123",
	}

	if action.ReplicaID != "replica-123" {
		t.Errorf("expected ReplicaID replica-123, got %s", action.ReplicaID)
	}

	action.isAction()
}
