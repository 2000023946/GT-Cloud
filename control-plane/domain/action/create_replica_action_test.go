package action

import "testing"

func TestCreateReplicaAction_IsAction(t *testing.T) {
	action := &CreateReplicaAction{
		DeploymentID: "deployment-123",
		Count:        3,
	}

	if action.DeploymentID != "deployment-123" {
		t.Errorf("expected DeploymentID %q, got %q", "deployment-123", action.DeploymentID)
	}

	if action.Count != 3 {
		t.Errorf("expected Count 3, got %d", action.Count)
	}

	if !action.isAction() {
		t.Error("expected isAction() to return true")
	}
}
