package replica

type ReplicaStatus string

const (
	Pending ReplicaStatus = "PENDING"
	Running ReplicaStatus = "RUNNING"
	Failed  ReplicaStatus = "FAILED"
	Stopped ReplicaStatus = "STOPPED"
)
