package action

import "testing"

func TestStopReplicaAction_IsAction(t *testing.T) {
	action := &StopReplicaAction{
		ReplicaID: "replica-123",
	}

	if action.ReplicaID != "replica-123" {
		t.Errorf("expected ReplicaID %q, got %q", "replica-123", action.ReplicaID)
	}

	if !action.isAction() {
		t.Error("expected isAction() to return true")
	}
}
