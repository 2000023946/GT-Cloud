package resource

import "testing"

func TestResource(t *testing.T) {
	r := Resource{
		CPU:    4,
		Memory: 8 * 1024 * 1024 * 1024,
	}

	if r.CPU != 4 {
		t.Errorf("expected CPU 4, got %d", r.CPU)
	}

	if r.Memory != 8*1024*1024*1024 {
		t.Errorf("expected memory 8 GB, got %d", r.Memory)
	}
}
