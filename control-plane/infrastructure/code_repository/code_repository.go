package code_repository

import (
	"github.com/google/uuid"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/application"
)

type CodeRepository struct {
	diskManager application.IDiskManager
}

func (s *CodeRepository) Save(codeFile application.IFile) (string, error) {

	// extract the file name and the file
	fileName := codeFile.Filename()
	//save the file to the disk of path job/id/file_name.
	// get teh file
	file, err := codeFile.Open()
	if err != nil {
		return "", err
	}
	defer file.Close()
	// get the content of the file
	content, err := s.diskManager.ReadAll(file)
	if err != nil {
		return "", err
	}
	filepath := "job/" + uuid.New().String() + "/" + fileName
	err = s.diskManager.WriteFile(filepath, content, 0644)
	if err != nil {
		return "", err
	}

	return filepath, nil
}
