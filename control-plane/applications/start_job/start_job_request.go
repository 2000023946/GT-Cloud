package start_job

import (
	"errors"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/application"
)

type StartJobRequest struct {
	Name           string            `json:"name"`
	StartCommand   string            `json:"start_command"`
	CodeFile       application.IFile `json:"code_file"`
	CpuResource    int               `json:"cpu_resource"`
	MemoryResource int               `json:"memory_resource"`
}

func (s *StartJobRequest) IsValid() error {
	if len(s.Name) == 0 {
		return errors.New("Name cannot be empty")
	}
	if len(s.StartCommand) == 0 {
		return errors.New("Start Command cannot be empty")
	}
	if s.CodeFile == nil || s.CodeFile.Size() == 0 {
		return errors.New("Code File cannot be empty")
	}
	return nil
}
