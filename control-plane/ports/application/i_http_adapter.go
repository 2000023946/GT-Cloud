package application

import "io"

type IHttpAdapter interface {
	Post(url string, contentType string, body io.Reader) (any, error)
}
