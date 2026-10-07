package action

import "testing"

func TestCreateReplicaActionImplementsAction(t *testing.T) {
	var action Action = CreateReplicaAction{
		DeploymentID: "deployment-123",
		Count:        2,
	}

	action.isAction()
}
