package update_system

import "io"

type UpdateSystemRequest struct {
	CodeZip   io.Reader
	ConfigZip io.Reader
}
