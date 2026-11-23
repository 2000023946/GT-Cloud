package http_adapter

import (
	"io"
	"net/http"
)

type HttpAdapter struct {
}

func (s *HttpAdapter) Post(url string, contentType string, body io.Reader) (any, error) {
	return http.Post(url, contentType, body)
}
