package action

type StopReplicaAction struct {
	ReplicaID string
}

func (s *StopReplicaAction) isAction() bool {
	return true
}
