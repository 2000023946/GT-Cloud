package endpoint

import "testing"

func TestEndpoint(t *testing.T) {
	e := Endpoint{
		ServiceID: "service-1",
		ReplicaID: "replica-1",
		Address:   "10.0.0.5",
		Port:      8080,
	}

	if e.ServiceID != "service-1" {
		t.Errorf("expected ServiceID service-1, got %s", e.ServiceID)
	}

	if e.ReplicaID != "replica-1" {
		t.Errorf("expected ReplicaID replica-1, got %s", e.ReplicaID)
	}

	if e.Address != "10.0.0.5" {
		t.Errorf("expected Address 10.0.0.5, got %s", e.Address)
	}

	if e.Port != 8080 {
		t.Errorf("expected Port 8080, got %d", e.Port)
	}
}
