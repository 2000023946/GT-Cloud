package action

type CreateReplicaAction struct {
	DeploymentID string
	Count        int
}

func (CreateReplicaAction) isAction() {}
