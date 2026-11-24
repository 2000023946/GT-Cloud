package application

import "io"

type HttpResponse struct {
	StatusCode int
	Body       []byte
}

type IHttpAdapter interface {
	Post(url string, contentType string, body io.Reader) (HttpResponse, error)
	Get(url string) (HttpResponse, error)
}
