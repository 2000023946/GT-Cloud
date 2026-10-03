package app

import "testing"

func TestApp(t *testing.T) {
	a := App{
		ID:       "app-1",
		CodePath: "/apps/app-1/",
		Command:  "python3 main.py",
	}

	if a.ID != "app-1" {
		t.Errorf("expected app ID app-1, got %s", a.ID)
	}

	if a.CodePath != "/apps/app-1/" {
		t.Errorf("expected code path /apps/app-1/, got %s", a.CodePath)
	}

	if a.Command != "python3 main.py" {
		t.Errorf("expected command python3 main.py, got %s", a.Command)
	}
}
