package application

import (
	"io"
	"os"
)

type IDiskManager interface {
	ReadAll(r io.Reader) ([]byte, error)
	WriteFile(name string, data []byte, perm os.FileMode) error
}
