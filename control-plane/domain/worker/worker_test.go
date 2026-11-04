package worker

import "testing"

func TestWorker(t *testing.T) {
	w := Worker{
		ID:      "worker-1",
		Address: "127.0.0.1:9000",
		Status:  StatusReady,
	}

	if w.ID != "worker-1" {
		t.Errorf("expected worker ID worker-1, got %s", w.ID)
	}

	if w.Status != StatusReady {
		t.Errorf("expected worker status READY, got %s", w.Status)
	}
}
