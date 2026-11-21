package data_plane

import (
	"bytes"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/globals"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/service"
	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/application"
)

type DataPlane struct {
	jsonAdapter application.IJsonAdapter
	httpAdapter application.IHttpAdapter
}

func (s *DataPlane) StartJob(service service.Service) error {

	// load and send the data
	payload, err := s.jsonAdapter.Marshal(service)
	if err != nil {
		return err
	}
	// don't really care about the reqeust object
	_, err = s.httpAdapter.Post(
		globals.DataPlaneUrl, "application/json", bytes.NewBuffer(payload))
	if err != nil {
		return err
	}
	return nil
}
