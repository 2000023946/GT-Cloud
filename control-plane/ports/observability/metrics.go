package observability

type Metrics interface {
	Increment(name string, value float64)
	Observe(name string, value float64)
}
