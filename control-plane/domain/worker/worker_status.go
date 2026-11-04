package worker

type WorkerStatus string

const (
	StatusUnknown WorkerStatus = "UNKNOWN"
	StatusReady   WorkerStatus = "READY"
	StatusBusy    WorkerStatus = "BUSY"
	StatusOffline WorkerStatus = "OFFLINE"
)
