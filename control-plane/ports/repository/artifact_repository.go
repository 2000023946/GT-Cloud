package repository

import "io"

type ArtifactRepository interface {
	Create(id string, artifact io.Reader) error
	Get(id string) (io.ReadCloser, error)
	Delete(id string) error
}
