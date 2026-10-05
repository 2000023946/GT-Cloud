package deployment

import "testing"

func TestDeployment(t *testing.T) {
	d := Deployment{
		ID:              "deployment-1",
		ServiceID:       "service-1",
		DesiredReplicas: 3,
	}

	if d.ID != "deployment-1" {
		t.Errorf("expected ID deployment-1, got %s", d.ID)
	}

	if d.ServiceID != "service-1" {
		t.Errorf("expected ServiceID service-1, got %s", d.ServiceID)
	}

	if d.DesiredReplicas != 3 {
		t.Errorf("expected DesiredReplicas 3, got %d", d.DesiredReplicas)
	}
}
