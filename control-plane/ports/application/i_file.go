package application

import "mime/multipart"

type IFile interface {
	Filename() string

	Open() (multipart.File, error)
	Size() int
}
