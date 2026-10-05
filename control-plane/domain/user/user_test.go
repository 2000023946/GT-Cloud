package user

import "testing"

func TestUser(t *testing.T) {
	u := User{
		ID:   "user-1",
		Name: "Mohamed",
	}

	if u.ID != "user-1" {
		t.Errorf("expected ID user-1, got %s", u.ID)
	}

	if u.Name != "Mohamed" {
		t.Errorf("expected Name Mohamed, got %s", u.Name)
	}
}
