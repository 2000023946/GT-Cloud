package replica

type Replica struct {
	ID           string
	DeploymentID string
	WorkerID     string
	Status       ReplicaStatus
}
