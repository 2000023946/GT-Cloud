package system

import "testing"

func TestSystem(t *testing.T) {
	s := System{
		ID:     "system-1",
		Name:   "my-app",
		UserID: "user-1",
	}

	if s.ID != "system-1" {
		t.Errorf("expected ID system-1, got %s", s.ID)
	}

	if s.Name != "my-app" {
		t.Errorf("expected Name my-app, got %s", s.Name)
	}

	if s.UserID != "user-1" {
		t.Errorf("expected UserID user-1, got %s", s.UserID)
	}
}
