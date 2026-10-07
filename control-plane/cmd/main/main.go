package main

import (
	"net/http"
)

var listenAndServe = http.ListenAndServe

func startServer(addr string) error {
	return listenAndServe(addr, nil)
}

func main() {
	_ = startServer(":8080")
}
