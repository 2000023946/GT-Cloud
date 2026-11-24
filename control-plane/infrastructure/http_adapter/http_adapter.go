package http_adapter

import (
	"io"
	"net/http"

	"github.gatech.edu/mabucar3/GT-Cloud/control-plane/ports/application"
)

type HttpAdapter struct {
	diskManager application.IDiskManager
}

func (s *HttpAdapter) Post(url string, contentType string, body io.Reader) (application.HttpResponse, error) {
	response, err := http.Post(url, contentType, body)
	if err != nil {
		return application.HttpResponse{}, err
	}
	defer response.Body.Close()
	rawBody, err := s.diskManager.ReadAll(response.Body)
	if err != nil {
		return application.HttpResponse{}, err
	}
	return application.HttpResponse{
		StatusCode: response.StatusCode,
		Body:       rawBody,
	}, nil
}

func (s *HttpAdapter) Get(url string) (application.HttpResponse, error) {
	response, err := http.Get(url)
	if err != nil {
		return application.HttpResponse{}, err
	}
	defer response.Body.Close()

	rawBody, err := s.diskManager.ReadAll(response.Body)
	if err != nil {
		return application.HttpResponse{}, err
	}

	return application.HttpResponse{
		StatusCode: response.StatusCode,
		Body:       rawBody,
	}, nil

}
