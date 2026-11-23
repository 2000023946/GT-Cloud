package action

type CreateReplicaAction struct {
	DeploymentID string
	Count        int
}

func (s *CreateReplicaAction) isAction() bool {
	return true
}
