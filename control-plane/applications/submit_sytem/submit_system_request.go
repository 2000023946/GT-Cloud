package submit_system

import "io"

type SubmitSystemRequest struct {
	CodeZip   io.Reader
	ConfigZip io.Reader
}
