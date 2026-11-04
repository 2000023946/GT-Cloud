package application

import (
	"io"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
)

type SystemParser interface {
	Parse(
		codeZip io.Reader,
		configZip io.Reader,
	) ([]service.Service, error)
}
