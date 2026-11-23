package disk_manager

import (
	"io"
	"os"
)

type DiskManager struct {
}

func (s *DiskManager) ReadAll(r io.Reader) ([]byte, error) {
	return io.ReadAll(r)
}

func (s *DiskManager) WriteFile(name string, data []byte, perm os.FileMode) error {
	return os.WriteFile(name, data, perm)
}
