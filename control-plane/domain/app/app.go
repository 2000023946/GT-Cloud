package app

import "github.com/google/uuid"

type App struct {
	ID       string
	CodePath string
	Command  string
}

func CreateApp(codepath string, command string) *App {
	return &App{
		ID:       uuid.New().String(),
		CodePath: codepath,
		Command:  command,
	}
}
