package repository

import (
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/application"
)

type ICodeRepository interface {
	// save the file and gives the code path
	Save(codeFile application.IFile) (string, error)
}
