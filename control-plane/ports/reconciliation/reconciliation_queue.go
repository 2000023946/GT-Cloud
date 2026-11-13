package reconciliation

type ReconciliationQueue interface {
	Enqueue(deploymentID string) error
	Dequeue() (string, error)
}
