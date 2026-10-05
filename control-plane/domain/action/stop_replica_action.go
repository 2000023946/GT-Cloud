package action

type StopReplicaAction struct {
	ReplicaID string
}

func (StopReplicaAction) isAction() {}
