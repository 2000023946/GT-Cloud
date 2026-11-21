package file

import "mime/multipart"

type File struct {
	File *multipart.FileHeader
}

func (s *File) FileName() string {
	return s.File.Filename
}

func (s *File) Open() (multipart.File, error) {
	return s.File.Open()
}

func (s *File) Size() int {
	return int(s.File.Size)
}
