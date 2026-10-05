package deployment

type Deployment struct {
	ID              string
	ServiceID       string
	DesiredReplicas int

	AutoScale    bool
	MinReplicas  int
	MaxReplicas  int
	CPUThreshold float64
}
